use std::io::Write as _;

pub fn handle_ecall(
    cpu: &mut crate::cpu::Cpu,
    mem: &mut crate::memory::GuestMemory,
) -> anyhow::Result<Option<i32>> {
    let nr = cpu.read_gpr(17);
    match nr {
        64 => {
            let fd = cpu.read_gpr(10) as i32;
            let buf_addr = cpu.read_gpr(11);
            let len = cpu.read_gpr(12) as usize;
            if fd == 1 || fd == 2 {
                let data = mem.read_bytes(buf_addr, len)?;
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                let written = handle.write(&data)?;
                let _ = handle.flush();
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
        94 => {
            let code = cpu.read_gpr(10) as i32;
            Ok(Some(code))
        }
        63 => {
            let fd = cpu.read_gpr(10) as i32;
            if fd == 0 {
                cpu.write_gpr(10, 0);
            } else {
                cpu.write_gpr(10, (-9i64) as u64);
            }
            Ok(None)
        }
        80 => {
            let fd = cpu.read_gpr(10) as i32;
            let statbuf = cpu.read_gpr(11);
            if fd == 0 || fd == 1 || fd == 2 {
                let mut st = [0u8; 128];
                let mode: u32 = 0x2000 | 0o666;
                let blksize: i32 = 1024;
                st[8..16].copy_from_slice(&1u64.to_le_bytes());
                st[16..20].copy_from_slice(&mode.to_le_bytes());
                st[20..24].copy_from_slice(&1u32.to_le_bytes());
                st[32..40].copy_from_slice(&0u64.to_le_bytes());
                st[56..60].copy_from_slice(&blksize.to_le_bytes());
                mem.write_bytes(statbuf, &st)?;
                cpu.write_gpr(10, 0);
            } else {
                cpu.write_gpr(10, (-9i64) as u64);
            }
            Ok(None)
        }
        160 => {
            let buf = cpu.read_gpr(10);
            let mut uts = [0u8; 65 * 6];
            let sys = b"Linux\0";
            for (i, &b) in sys.iter().enumerate() {
                uts[i] = b;
            }
            let rel = b"5.0.0\0";
            for (i, &b) in rel.iter().enumerate() {
                uts[65 + i] = b;
            }
            let mach = b"riscv64\0";
            for (i, &b) in mach.iter().enumerate() {
                uts[65 * 4 + i] = b;
            }
            mem.write_bytes(buf, &uts)?;
            cpu.write_gpr(10, 0);
            Ok(None)
        }
        214 => {
            let addr = cpu.read_gpr(10);
            if addr == 0 {
                cpu.write_gpr(10, cpu.brk);
            } else if addr <= (1u64 << 28) {
                cpu.brk = addr;
                cpu.write_gpr(10, addr);
            } else {
                cpu.write_gpr(10, (-12i64) as u64);
            }
            Ok(None)
        }
        29 => {
            let fd = cpu.read_gpr(10) as i32;
            let cmd = cpu.read_gpr(11);
            let arg = cpu.read_gpr(12);
            if fd == 0 || fd == 1 || fd == 2 {
                if cmd == 0x5401 && arg != 0 {
                    let t = [0u8; 60];
                    mem.write_bytes(arg, &t)?;
                }
                cpu.write_gpr(10, 0);
            } else {
                cpu.write_gpr(10, (-25i64) as u64);
            }
            Ok(None)
        }
        56 => {
            cpu.write_gpr(10, (-2i64) as u64);
            Ok(None)
        }
        57 => {
            let fd = cpu.read_gpr(10) as i32;
            if fd >= 0 {
                cpu.write_gpr(10, 0);
            } else {
                cpu.write_gpr(10, (-9i64) as u64);
            }
            Ok(None)
        }
        66 => {
            let fd = cpu.read_gpr(10) as i32;
            let iovp = cpu.read_gpr(11);
            let iovcnt = cpu.read_gpr(12) as usize;
            if fd == 1 || fd == 2 {
                let mut total: u64 = 0;
                for i in 0..iovcnt {
                    let base = mem.read_u64(iovp + (i as u64) * 16)?;
                    let len = mem.read_u64(iovp + (i as u64) * 16 + 8)? as usize;
                    if len > 0 {
                        let data = mem.read_bytes(base, len)?;
                        let stdout = std::io::stdout();
                        let mut handle = stdout.lock();
                        let _ = handle.write_all(&data);
                        let _ = handle.flush();
                        total += len as u64;
                    }
                }
                cpu.write_gpr(10, total);
            } else {
                cpu.write_gpr(10, u64::MAX);
            }
            Ok(None)
        }
        222 => {
            let addr = cpu.read_gpr(10);
            let len = cpu.read_gpr(11);
            let mut ret = addr;
            if ret == 0 {
                ret = 0x30000000u64;
            }
            if len > (1u64 << 28) {
                ret = (-12i64) as u64;
            }
            cpu.write_gpr(10, ret);
            Ok(None)
        }
        _ => {
            cpu.write_gpr(10, (-38i64) as u64);
            Ok(None)
        }
    }
}
