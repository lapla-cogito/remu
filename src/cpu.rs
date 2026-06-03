#[derive(Default)]
pub struct Cpu {
    pub gpr: [u64; 32],
    pub pc: u64,
    pub fpr: [u64; 32],
    pub fcsr: u32,
}

impl Cpu {
    pub fn new(entry: u64) -> Self {
        let mut c = <Self as std::default::Default>::default();
        c.pc = entry;
        c
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
}
