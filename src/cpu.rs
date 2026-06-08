#[repr(C)]
#[derive(Default)]
pub struct Cpu {
    pub gpr: [u64; 32],
    pub pc: u64,
    pub fpr: [u64; 32],
    pub fcsr: u32,
    _pad: u32,
    pub reservation_addr: u64,
    pub reservation_size: u64,
    pub brk: u64,
    pub priv_mode: u64,
    pub mstatus: u64,
    pub mepc: u64,
    pub satp: u64,
    pub medeleg: u64,
    pub mideleg: u64,
    pub csr: hashbrown::HashMap<u16, u64>,
    pub sepc: u64,
    pub mtvec: u64,
    pub stvec: u64,
    pub mcause: u64,
    pub scause: u64,
    pub mtval: u64,
    pub stval: u64,
}

impl Cpu {
    pub fn new(entry: u64) -> Self {
        crate::cpu::Cpu {
            pc: entry,
            brk: 0x200000u64,
            priv_mode: 3,
            mstatus: 0,
            mepc: 0,
            satp: 0,
            medeleg: 0,
            mideleg: 0,
            sepc: 0,
            mtvec: 0,
            stvec: 0,
            mcause: 0,
            scause: 0,
            mtval: 0,
            stval: 0,
            ..<Self as std::default::Default>::default()
        }
    }

    pub fn read_gpr(&self, reg: u8) -> u64 {
        if reg == 0 { 0 } else { self.gpr[reg as usize] }
    }

    pub fn write_gpr(&mut self, reg: u8, val: u64) {
        if reg != 0 {
            self.gpr[reg as usize] = val;
        }
    }

    pub fn read_fpr(&self, reg: u8) -> u64 {
        self.fpr[reg as usize]
    }

    pub fn write_fpr(&mut self, reg: u8, val: u64) {
        self.fpr[reg as usize] = val;
    }

    pub fn read_fpr_s(&self, reg: u8) -> u32 {
        self.fpr[reg as usize] as u32
    }

    pub fn write_fpr_s(&mut self, reg: u8, val: u32) {
        self.fpr[reg as usize] = (val as u64) | 0xffffffff00000000u64;
    }

    pub fn set_reservation(&mut self, addr: u64, size: u64) {
        self.reservation_addr = addr;
        self.reservation_size = size;
    }

    pub fn clear_reservation_if_overlap(&mut self, addr: u64, size: u64) {
        if self.reservation_size != 0 {
            let res_end = self.reservation_addr.wrapping_add(self.reservation_size);
            let store_end = addr.wrapping_add(size);
            if self.reservation_addr < store_end && addr < res_end {
                self.reservation_addr = 0;
                self.reservation_size = 0;
            }
        }
    }

    pub fn check_and_clear_reservation(&mut self, addr: u64, size: u64) -> bool {
        if self.reservation_addr == addr && self.reservation_size == size {
            self.reservation_addr = 0;
            self.reservation_size = 0;
            true
        } else {
            self.reservation_addr = 0;
            self.reservation_size = 0;
            false
        }
    }

    pub fn read_csr(&self, csr: u16) -> u64 {
        match csr {
            0xf14 => 0,
            0x105 => self.stvec,
            0x141 => self.sepc,
            0x142 => self.scause,
            0x143 => self.stval,
            0x180 => self.satp,
            0x300 => self.mstatus,
            0x302 => self.medeleg,
            0x303 => self.mideleg,
            0x305 => self.mtvec,
            0x341 => self.mepc,
            0x342 => self.mcause,
            0x343 => self.mtval,
            _ => *self.csr.get(&csr).unwrap_or(&0),
        }
    }

    pub fn write_csr(&mut self, csr: u16, val: u64) {
        match csr {
            0x105 => {
                self.stvec = val;
            }
            0x141 => {
                self.sepc = val;
            }
            0x142 => {
                self.scause = val;
            }
            0x143 => {
                self.stval = val;
            }
            0x180 => {
                self.satp = val;
            }
            0x300 => {
                self.mstatus = val;
            }
            0x302 => {
                self.medeleg = val;
            }
            0x303 => {
                self.mideleg = val;
            }
            0x305 => {
                self.mtvec = val;
            }
            0x341 => {
                self.mepc = val;
            }
            0x342 => {
                self.mcause = val;
            }
            0x343 => {
                self.mtval = val;
            }
            _ => {
                self.csr.insert(csr, val);
            }
        }
    }

    /// Take a synchronous exception (e.g. ecall).
    /// Sets the appropriate xepc/xcause/xtval, updates mstatus bits for previous
    /// priv, computes the trap vector from xtvec (direct mode), changes priv_mode
    /// and pc. Delegation via medeleg is considered (M vs S).
    pub fn take_exception(&mut self, cause: u64, tval: u64) {
        let current_priv = self.priv_mode;
        let deleg = (self.medeleg & (1u64 << (cause & 0x1f))) != 0;
        let target_priv = if deleg && current_priv != 3 { 1 } else { 3 };

        if target_priv == 3 {
            // trap to M-mode
            let mpp = current_priv;
            self.mstatus = (self.mstatus & !(3u64 << 11)) | (mpp << 11);
            let mie = (self.mstatus >> 3) & 1;
            self.mstatus = (self.mstatus & !(1u64 << 7)) | (mie << 7);
            self.mstatus &= !(1u64 << 3);
            self.mepc = self.pc;
            self.mcause = cause;
            self.mtval = tval;
            let vec = self.mtvec & !1u64;
            self.pc = vec;
            self.priv_mode = 3;
        } else {
            // trap to S-mode
            let spp = current_priv & 1; // 0 or 1
            self.mstatus = (self.mstatus & !(1u64 << 8)) | (spp << 8);
            let sie = (self.mstatus >> 1) & 1;
            self.mstatus = (self.mstatus & !(1u64 << 5)) | (sie << 5);
            self.mstatus &= !(1u64 << 1);
            self.sepc = self.pc;
            self.scause = cause;
            self.stval = tval;
            let vec = self.stvec & !1u64;
            self.pc = vec;
            self.priv_mode = 1;
        }
    }
}
