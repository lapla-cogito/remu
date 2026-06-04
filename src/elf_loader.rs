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
    for ph in &elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            let off = ph.p_offset as usize;
            let fsz = ph.p_filesz as usize;
            let vaddr = ph.p_vaddr;
            let msz = ph.p_memsz as usize;
            for i in 0..fsz {
                mem.write_u8(vaddr + i as u64, file_data[off + i])?;
            }
            for i in fsz..msz {
                mem.write_u8(vaddr + i as u64, 0)?;
            }
        }
    }
    cpu.brk = 0x200000u64;
    let mut gp = 0u64;
    for sym in &elf.syms {
        if let Some(name) = elf.strtab.get_at(sym.st_name)
            && name == "__global_pointer$"
        {
            gp = sym.st_value;
            break;
        }
    }
    cpu.write_gpr(3, gp);
    let stack_top = 0x0800_0000u64;
    let sp = setup_minimal_stack(mem, stack_top, &elf)?;
    cpu.write_gpr(2, sp);
    cpu.pc = elf.header.e_entry;
    Ok(())
}

fn push_u64(mem: &mut crate::memory::GuestMemory, mut sp: u64, v: u64) -> anyhow::Result<u64> {
    sp -= 8;
    mem.write_u64(sp, v)?;
    Ok(sp)
}

fn setup_minimal_stack(
    mem: &mut crate::memory::GuestMemory,
    mut sp: u64,
    elf: &goblin::elf::Elf,
) -> anyhow::Result<u64> {
    let prog_name = b"a.out\0";
    sp -= prog_name.len() as u64;
    let argv0 = sp;
    for (i, &b) in prog_name.iter().enumerate() {
        mem.write_u8(sp + i as u64, b)?;
    }
    let auxs: &[(u64, u64)] = &[
        (3, elf.header.e_phoff),
        (4, elf.header.e_phentsize as u64),
        (5, elf.header.e_phnum as u64),
        (6, 4096),
        (9, elf.header.e_entry),
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
