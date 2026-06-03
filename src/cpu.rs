#[derive(Default)]
#[expect(dead_code)]
pub struct Cpu {
    pub gpr: [u64; 32],
    pub pc: u64,
    pub fpr: [u64; 32],
    pub fcsr: u32,
}

impl Cpu {
    pub fn new(entry: u64) -> Self {
        crate::cpu::Cpu {
            pc: entry,
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
}
