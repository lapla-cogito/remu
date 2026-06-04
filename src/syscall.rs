use std::io::Write as _;

pub fn handle_ecall(
    cpu: &mut crate::cpu::Cpu,
    mem: &mut crate::memory::GuestMemory,
) -> anyhow::Result<Option<i32>> {
    let nr = cpu.read_gpr(17);
    match nr {
        64 => {
            // write
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
            // exit
            let code = cpu.read_gpr(10) as i32;
            Ok(Some(code))
        }
        94 => {
            // exit_group
            let code = cpu.read_gpr(10) as i32;
            Ok(Some(code))
        }
        63 => {
            // read
            let fd = cpu.read_gpr(10) as i32;
            if fd == 0 {
                cpu.write_gpr(10, 0);
            } else {
                cpu.write_gpr(10, (-9i64) as u64);
            }
            Ok(None)
        }
        80 => {
            // fstat
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
            // uname
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
            // brk
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
            // ioctl
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
            // openat
            cpu.write_gpr(10, (-2i64) as u64);
            Ok(None)
        }
        57 => {
            // close
            let fd = cpu.read_gpr(10) as i32;
            if fd >= 0 {
                cpu.write_gpr(10, 0);
            } else {
                cpu.write_gpr(10, (-9i64) as u64);
            }
            Ok(None)
        }
        66 => {
            // writev
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
            // mmap
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
        78 => {
            // readlinkat
            let path_addr = cpu.read_gpr(11);
            let buf = cpu.read_gpr(12);
            let bufsiz = cpu.read_gpr(13) as usize;
            let mut path = vec![];
            for i in 0..256 {
                let b = mem.read_u8(path_addr + i)?;
                if b == 0 {
                    break;
                }
                path.push(b);
            }
            let pstr = String::from_utf8_lossy(&path);
            let val = if pstr.starts_with("/proc/se") {
                let fake = b"remu";
                let n = std::cmp::min(fake.len(), bufsiz);
                mem.write_bytes(buf, &fake[..n])?;
                n as i64
            } else {
                -2 // ENOENT
            };
            cpu.write_gpr(10, val as u64);
            Ok(None)
        }
        96 => {
            // set_tid_address
            cpu.write_gpr(10, 1);
            Ok(None)
        }
        99 => {
            // set_robust_list
            cpu.write_gpr(10, 0);
            Ok(None)
        }
        172 => {
            // getpid
            cpu.write_gpr(10, 1);
            Ok(None)
        }
        226 => {
            // mprotect
            cpu.write_gpr(10, 0);
            Ok(None)
        }
        261 => {
            // prlimit64
            let oldp = cpu.read_gpr(13);
            if oldp != 0 {
                let mut r = [0u8; 16];
                let cur: u64 = 8 * 1024 * 1024;
                let max: u64 = u64::MAX;
                r[0..8].copy_from_slice(&cur.to_le_bytes());
                r[8..16].copy_from_slice(&max.to_le_bytes());
                mem.write_bytes(oldp, &r)?;
            }
            cpu.write_gpr(10, 0);
            Ok(None)
        }
        278 => {
            // getrandom
            let buf = cpu.read_gpr(10);
            let len = cpu.read_gpr(11) as usize;
            for i in 0..len {
                mem.write_u8(buf + i as u64, (0xA5u8).wrapping_add(i as u8))?;
            }
            cpu.write_gpr(10, len as u64);
            Ok(None)
        }
        _ => {
            eprintln!("unhandled syscall nr={}", nr);
            cpu.write_gpr(10, (-38i64) as u64);
            Ok(None)
        }
    }
}
