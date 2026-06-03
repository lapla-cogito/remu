pub fn handle_ecall(cpu: &mut crate::cpu::Cpu, mem: &mut crate::memory::GuestMemory) -> anyhow::Result<Option<i32>> {
    let nr = cpu.read_gpr(17);
    match nr {
        64 => {
            let fd = cpu.read_gpr(10) as i32;
            let buf_addr = cpu.read_gpr(11);
            let len = cpu.read_gpr(12) as usize;
            if fd == 1 || fd == 2 {
                let mut data = vec![0u8; len];
                for i in 0..len {
                    data[i] = mem.read_u8(buf_addr + i as u64)?;
                }
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                let written = std::io::Write::write(&mut handle, &data)?;
                let _ = std::io::Write::flush(&mut handle);
                cpu.write_gpr(10, written as u64);
            } else {
                cpu.write_gpr(10, u64::MAX);
            }
            Ok(None)
        }
        93 => {
            let code = cpu.read_gpr(10) as i32;
            Ok(Some(code))
        }
        214 => {
            let addr = cpu.read_gpr(10);
            cpu.write_gpr(10, addr);
            Ok(None)
        }
        _ => {
            cpu.write_gpr(10, (-38i64) as u64);
            Ok(None)
        }
    }
}