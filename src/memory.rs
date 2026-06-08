pub struct GuestMemory {
    pages: hashbrown::HashMap<u64, [u8; 4096]>,
    // Highest address written or ensured so far.
    max_touched: u64,

    jit_mmap_base: *mut u8,
    jit_mmap_size: usize,
}

impl GuestMemory {
    const PAGE_SIZE: u64 = 4096;

    pub fn new() -> Self {
        // Sparse by default; grows only for touched pages. Enables practical high-VA ELFs.
        Self {
            pages: hashbrown::HashMap::new(),
            max_touched: 0,
            jit_mmap_base: std::ptr::null_mut(),
            jit_mmap_size: 0,
        }
    }

    fn page_num(addr: u64) -> u64 {
        addr / Self::PAGE_SIZE
    }

    fn page_off(addr: u64) -> usize {
        (addr & (Self::PAGE_SIZE - 1)) as usize
    }

    pub fn ensure(&mut self, addr: u64) {
        // Ensure the page for addr exists (for high VA this is cheap: one 4K alloc).
        if addr > self.max_touched {
            self.max_touched = addr;
        }
        let p = Self::page_num(addr);
        self.pages.entry(p).or_insert_with(|| [0u8; 4096]);
    }

    pub fn read_u8(&self, addr: u64) -> anyhow::Result<u8> {
        let p = Self::page_num(addr);
        let off = Self::page_off(addr);
        if let Some(page) = self.pages.get(&p) {
            Ok(page[off])
        } else {
            // Never-written pages (or beyond) read as 0. Matches previous semantics for
            // sparse/high-linked guests and unmapped MMIO regions.
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
        if addr > self.max_touched {
            self.max_touched = addr;
        }
        let p = Self::page_num(addr);
        let off = Self::page_off(addr);
        let page = self.pages.entry(p).or_insert_with(|| [0u8; 4096]);
        page[off] = val;
        // If JIT mmap region is active, mirror the write so direct [base + gva]
        // accesses from emitted code see up-to-date data.
        if !self.jit_mmap_base.is_null() && (addr as usize) < self.jit_mmap_size {
            unsafe {
                *self.jit_mmap_base.add(addr as usize) = val;
            }
        }
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
        if self.jit_mmap_base.is_null() {
            let size: usize = 1usize << 34; // 16 GiB virtual reservation
            let ptr = unsafe {
                libc::mmap(
                    std::ptr::null_mut::<libc::c_void>(),
                    size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
                    -1,
                    0,
                )
            };
            if ptr == libc::MAP_FAILED {
                let cover = self.max_touched.saturating_add(1 << 20);
                let need = if cover == 0 { 1 << 20 } else { cover as usize };
                let v = std::vec![0u8; need];
                let base = v.as_ptr() as *mut u8;
                std::mem::forget(v);
                self.jit_mmap_base = base;
                self.jit_mmap_size = need;
            } else {
                self.jit_mmap_base = ptr as *mut u8;
                self.jit_mmap_size = size;
            }
            // Copy current sparse pages into the region at their guest VA.
            for (&pnum, page) in self.pages.iter() {
                let va0 = pnum * Self::PAGE_SIZE;
                if (va0 as usize) < self.jit_mmap_size {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            page.as_ptr(),
                            self.jit_mmap_base.add(va0 as usize),
                            Self::PAGE_SIZE as usize,
                        );
                    }
                }
            }
        }
        self.jit_mmap_base
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

impl std::ops::Drop for GuestMemory {
    fn drop(&mut self) {
        if !self.jit_mmap_base.is_null() && self.jit_mmap_size > 0 {
            unsafe {
                let _ = libc::munmap(self.jit_mmap_base as *mut libc::c_void, self.jit_mmap_size);
            }
        }
    }
}
