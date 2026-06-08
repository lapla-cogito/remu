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

    #[expect(dead_code)]
    pub fn read_fcsr(&self) -> u32 {
        self.fcsr
    }

    #[expect(dead_code)]
    pub fn write_fcsr(&mut self, val: u32) {
        self.fcsr = val & 0xff;
    }

    #[expect(dead_code)]
    pub fn frm(&self) -> u8 {
        ((self.fcsr >> 5) & 7) as u8
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
            0x180 => self.satp,
            0x300 => self.mstatus,
            0x302 => self.medeleg,
            0x303 => self.mideleg,
            0x341 => self.mepc,
            _ => *self.csr.get(&csr).unwrap_or(&0),
        }
    }

    pub fn write_csr(&mut self, csr: u16, val: u64) {
        match csr {
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
            0x341 => {
                self.mepc = val;
            }
            _ => {
                self.csr.insert(csr, val);
            }
        }
    }
}
