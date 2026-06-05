pub struct GuestMemory {
    data: Vec<u8>,
}

impl GuestMemory {
    pub fn new() -> Self {
        // Starts small and grows on demand to support guest programs using high addresses
        // or large memory regions (common for some bare-metal and test ELFs).
        Self {
            data: ::std::vec::Vec::new(),
        }
    }

    pub fn ensure(&mut self, addr: u64) {
        let idx = addr as usize;
        if idx >= self.data.len() {
            let new_len = (idx + 1).next_power_of_two().max(self.data.len());
            self.data.resize(new_len, 0);
        }
    }

    pub fn read_u8(&self, addr: u64) -> anyhow::Result<u8> {
        let i = addr as usize;
        if i < self.data.len() {
            Ok(self.data[i])
        } else {
            // Never-written addresses (including beyond current grow) read as 0.
            // This preserves semantics for sparse/high-linked guests without forcing full prealloc.
            Ok(0)
        }
    }

    pub fn read_u16(&self, addr: u64) -> anyhow::Result<u16> {
        let b0 = self.read_u8(addr)? as u16;
        let b1 = self.read_u8(addr.wrapping_add(1))? as u16;
        Ok(b0 | (b1 << 8))
    }

    pub fn read_u32(&self, addr: u64) -> anyhow::Result<u32> {
        let lo = self.read_u16(addr)? as u32;
        let hi = self.read_u16(addr.wrapping_add(2))? as u32;
        Ok(lo | (hi << 16))
    }

    pub fn read_u64(&self, addr: u64) -> anyhow::Result<u64> {
        let lo = self.read_u32(addr)? as u64;
        let hi = self.read_u32(addr.wrapping_add(4))? as u64;
        Ok(lo | (hi << 32))
    }

    pub fn write_u8(&mut self, addr: u64, val: u8) -> anyhow::Result<()> {
        self.ensure(addr);
        let i = addr as usize;
        self.data[i] = val;
        Ok(())
    }

    pub fn write_u16(&mut self, addr: u64, val: u16) -> anyhow::Result<()> {
        self.write_u8(addr, val as u8)?;
        self.write_u8(addr.wrapping_add(1), (val >> 8) as u8)
    }

    pub fn write_u32(&mut self, addr: u64, val: u32) -> anyhow::Result<()> {
        self.write_u16(addr, val as u16)?;
        self.write_u16(addr.wrapping_add(2), (val >> 16) as u16)
    }

    pub fn write_u64(&mut self, addr: u64, val: u64) -> anyhow::Result<()> {
        self.write_u32(addr, val as u32)?;
        self.write_u32(addr.wrapping_add(4), (val >> 32) as u32)
    }

    pub fn mem_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }

    pub fn read_bytes(&self, addr: u64, len: usize) -> anyhow::Result<Vec<u8>> {
        let mut v = vec![0u8; len];
        for (i, b) in v.iter_mut().enumerate() {
            *b = self.read_u8(addr + i as u64)?;
        }
        Ok(v)
    }

    pub fn write_bytes(&mut self, addr: u64, data: &[u8]) -> anyhow::Result<()> {
        for (i, &b) in data.iter().enumerate() {
            self.write_u8(addr + i as u64, b)?;
        }
        Ok(())
    }
}
