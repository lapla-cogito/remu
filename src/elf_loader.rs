pub fn load_elf(
    path: &str,
    cpu: &mut crate::cpu::Cpu,
    mem: &mut crate::memory::GuestMemory,
) -> anyhow::Result<()> {
    let file_data = std::fs::read(path)?;
    let elf = goblin::elf::Elf::parse(&file_data)?;
    if !elf.is_64 {
        anyhow::bail!("expected 64-bit ELF");
    }
    if elf.header.e_machine != goblin::elf::header::EM_RISCV {
        anyhow::bail!("expected RISC-V ELF");
    }
    let load_base: u64 = if elf.header.e_type == goblin::elf::header::ET_DYN {
        0x400000u64
    } else {
        0
    };
    for ph in &elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            let off = ph.p_offset as usize;
            let fsz = ph.p_filesz as usize;
            let vaddr = load_base + ph.p_vaddr;
            let msz = ph.p_memsz as usize;
            for i in 0..fsz {
                mem.write_u8(vaddr + i as u64, file_data[off + i])?;
            }
            for i in fsz..msz {
                mem.write_u8(vaddr + i as u64, 0)?;
            }
        }
    }
    // Ensure room for stack growth etc. beyond the loaded segments (some ELFs link at high addresses).
    let mut max_va: u64 = 0;
    for ph in &elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            let end = load_base + ph.p_vaddr + ph.p_memsz;
            if end > max_va {
                max_va = end;
            }
        }
    }
    if max_va > 0 {
        mem.ensure(max_va + (1 << 17));
    }
    cpu.brk = 0x200000u64;
    let mut gp = 0u64;
    for sym in &elf.syms {
        if let Some(name) = elf.strtab.get_at(sym.st_name)
            && name == "__global_pointer$"
        {
            gp = load_base + sym.st_value;
            break;
        }
    }
    cpu.write_gpr(3, gp);
    // Place the auxiliary stack at a low address. This is only used for initial
    // argc/argv/auxv setup passed to _start. High-linked bare-metal programs
    // (e.g. kernels) typically set their own stack pointer from their own bss
    // and ignore this value.
    const AUX_STACK_TOP: u64 = 0x100000;
    let sp = setup_minimal_stack(mem, AUX_STACK_TOP, &elf, load_base)?;
    cpu.write_gpr(2, sp);

    for ph in &elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_TLS {
            let template_va = load_base + ph.p_vaddr;
            let filesz = ph.p_filesz;
            let memsz = ph.p_memsz;
            // Place the initial TLS block at a low address that does not conflict
            // with either typical small static binaries or high-linked kernels.
            const INITIAL_TLS_BASE: u64 = 0x200000;
            mem.ensure(INITIAL_TLS_BASE + memsz + 0x1000);
            for i in 0..filesz {
                let b = mem.read_u8(template_va + i)?;
                mem.write_u8(INITIAL_TLS_BASE + i, b)?;
            }
            for i in filesz..memsz {
                mem.write_u8(INITIAL_TLS_BASE + i, 0)?;
            }
            cpu.write_gpr(4, INITIAL_TLS_BASE);
            break;
        }
    }

    apply_relocations(mem, &elf, load_base, cpu)?;

    cpu.pc = load_base + elf.header.e_entry;
    Ok(())
}

fn push_u64(mem: &mut crate::memory::GuestMemory, mut sp: u64, v: u64) -> anyhow::Result<u64> {
    sp -= 8;
    mem.write_u64(sp, v)?;
    Ok(sp)
}

fn apply_relocations(
    mem: &mut crate::memory::GuestMemory,
    elf: &goblin::elf::Elf,
    load_base: u64,
    cpu: &mut crate::cpu::Cpu,
) -> anyhow::Result<()> {
    const R_RISCV_32: u32 = 1;
    const R_RISCV_64: u32 = 2;
    const R_RISCV_RELATIVE: u32 = 3;
    const R_RISCV_IRELATIVE: u32 = 6;

    for rel in elf.dynrelas.iter().chain(elf.pltrelocs.iter()) {
        let addr = load_base + rel.r_offset;
        let addend = rel.r_addend.unwrap_or(0);
        match rel.r_type {
            R_RISCV_RELATIVE => {
                let val = load_base.wrapping_add(addend as u64);
                mem.write_u64(addr, val)?;
            }
            R_RISCV_64 => {
                let sym = match elf.syms.get(rel.r_sym) {
                    Some(s) => s,
                    None => continue,
                };
                let sym_val = load_base.wrapping_add(sym.st_value);
                let val = sym_val.wrapping_add(addend as u64);
                mem.write_u64(addr, val)?;
            }
            R_RISCV_32 => {
                let sym = match elf.syms.get(rel.r_sym) {
                    Some(s) => s,
                    None => continue,
                };
                let sym_val = load_base.wrapping_add(sym.st_value);
                let val = sym_val.wrapping_add(addend as u64) as u32;
                mem.write_u32(addr, val)?;
            }
            R_RISCV_IRELATIVE => {
                let resolver = load_base.wrapping_add(addend as u64);
                let resolved = resolve_ifunc(cpu, mem, resolver)?;
                mem.write_u64(addr, resolved)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn resolve_ifunc(
    cpu: &mut crate::cpu::Cpu,
    mem: &mut crate::memory::GuestMemory,
    resolver: u64,
) -> anyhow::Result<u64> {
    let saved_pc = cpu.pc;
    let saved_ra = cpu.read_gpr(1);
    let saved_sp = cpu.read_gpr(2);
    // Sentinel return address (will never be valid code in normal use).
    let magic_ra: u64 = 0x1;
    cpu.write_gpr(1, magic_ra);
    cpu.pc = resolver;

    let mut steps: u32 = 0;
    while steps < 10000 {
        if cpu.pc == magic_ra {
            let ret = cpu.read_gpr(10);
            cpu.pc = saved_pc;
            cpu.write_gpr(1, saved_ra);
            cpu.write_gpr(2, saved_sp);
            return Ok(ret);
        }
        crate::interp::step(cpu, mem)?;
        steps += 1;
    }
    // Restore before bailing so state is not left corrupted.
    cpu.pc = saved_pc;
    cpu.write_gpr(1, saved_ra);
    cpu.write_gpr(2, saved_sp);
    anyhow::bail!("ifunc resolver did not return in time")
}

fn setup_minimal_stack(
    mem: &mut crate::memory::GuestMemory,
    mut sp: u64,
    elf: &goblin::elf::Elf,
    load_base: u64,
) -> anyhow::Result<u64> {
    let prog_name = b"a.out\0";
    sp -= prog_name.len() as u64;
    let argv0 = sp;
    for (i, &b) in prog_name.iter().enumerate() {
        mem.write_u8(sp + i as u64, b)?;
    }
    let phdr = load_base + elf.header.e_phoff;
    let entry = load_base + elf.header.e_entry;
    let auxs: &[(u64, u64)] = &[
        (3, phdr),
        (4, elf.header.e_phentsize as u64),
        (5, elf.header.e_phnum as u64),
        (6, 4096),
        (9, entry),
        (0, 0),
    ];
    for (typ, val) in auxs.iter().rev() {
        sp = push_u64(mem, sp, *val)?;
        sp = push_u64(mem, sp, *typ)?;
    }
    sp = push_u64(mem, sp, 0)?;
    sp = push_u64(mem, sp, 0)?;
    sp = push_u64(mem, sp, argv0)?;
    sp = push_u64(mem, sp, 1)?;
    Ok(sp)
}
