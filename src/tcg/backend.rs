#[expect(clippy::manual_checked_ops, clippy::if_same_then_else)]
pub fn execute_tcg(
    ctx: &crate::tcg::context::TcgContext,
    cpu: &mut crate::cpu::Cpu,
    mem: &mut crate::memory::GuestMemory,
) -> Option<u64> {
    let num_t = ctx.num_temps() as usize;
    let mut temps: Vec<u64> = vec![0u64; num_t];
    let mut next_pc: Option<u64> = None;
    // build label to op index map
    let mut label_pos: Vec<Option<usize>> = vec![None; (ctx.num_labels() as usize).max(1)];
    for (i, op) in ctx.ops.iter().enumerate() {
        if let crate::tcg::op::TcgOpcode::SetLabel = op.opc
            && let crate::tcg::op::TcgArg::Label(l) = &op.args[0]
            && (*l as usize) < label_pos.len()
        {
            label_pos[*l as usize] = Some(i);
        }
    }
    let mut i: usize = 0;
    while i < ctx.ops.len() {
        let op = &ctx.ops[i];
        match op.opc {
            crate::tcg::op::TcgOpcode::MovI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    temps[*d as usize] = temps[*s as usize];
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c)) =
                    (&op.args[0], &op.args[1])
                {
                    temps[*d as usize] = *c;
                }
            }
            crate::tcg::op::TcgOpcode::AddI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize].wrapping_add(temps[*s2 as usize]);
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize].wrapping_add(*c);
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = *c1 + temps[*s2 as usize];
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = *c1 + *c2;
                }
            }
            crate::tcg::op::TcgOpcode::SubI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize].wrapping_sub(temps[*s2 as usize]);
                }
            }
            crate::tcg::op::TcgOpcode::AndI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize] & temps[*s2 as usize];
                }
            }
            crate::tcg::op::TcgOpcode::OrI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize] | temps[*s2 as usize];
                }
            }
            crate::tcg::op::TcgOpcode::XorI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize] ^ temps[*s2 as usize];
                }
            }
            crate::tcg::op::TcgOpcode::ShlI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize] << (temps[*s2 as usize] & 63);
                }
            }
            crate::tcg::op::TcgOpcode::ShrI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize] >> (temps[*s2 as usize] & 63);
                }
            }
            crate::tcg::op::TcgOpcode::SarI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] =
                        (temps[*s1 as usize] as i64 >> (temps[*s2 as usize] & 63)) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::MulI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    temps[*d as usize] = temps[*s1 as usize].wrapping_mul(temps[*s2 as usize]);
                }
            }
            crate::tcg::op::TcgOpcode::MulhI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = temps[*s1 as usize] as i64 as i128;
                    let b = temps[*s2 as usize] as i64 as i128;
                    temps[*d as usize] = ((a * b) >> 64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::MulhsuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = temps[*s1 as usize] as i64 as i128;
                    let b = temps[*s2 as usize] as u64 as i128;
                    temps[*d as usize] = ((a * b) >> 64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::MulhuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = temps[*s1 as usize] as u64 as u128;
                    let b = temps[*s2 as usize] as u64 as u128;
                    temps[*d as usize] = ((a * b) >> 64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::DivsI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let dividend = temps[*s1 as usize] as i64;
                    let divisor = temps[*s2 as usize] as i64;
                    temps[*d as usize] = if divisor == 0 {
                        -1i64 as u64
                    } else if dividend == i64::MIN && divisor == -1 {
                        i64::MIN as u64
                    } else {
                        (dividend / divisor) as u64
                    };
                }
            }
            crate::tcg::op::TcgOpcode::DivuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let dividend = temps[*s1 as usize];
                    let divisor = temps[*s2 as usize];
                    temps[*d as usize] = if divisor == 0 {
                        u64::MAX
                    } else {
                        dividend / divisor
                    };
                }
            }
            crate::tcg::op::TcgOpcode::RemsI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let dividend = temps[*s1 as usize] as i64;
                    let divisor = temps[*s2 as usize] as i64;
                    temps[*d as usize] = if divisor == 0 {
                        dividend as u64
                    } else if dividend == i64::MIN && divisor == -1 {
                        0
                    } else {
                        (dividend % divisor) as u64
                    };
                }
            }
            crate::tcg::op::TcgOpcode::RemuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let dividend = temps[*s1 as usize];
                    let divisor = temps[*s2 as usize];
                    temps[*d as usize] = if divisor == 0 {
                        dividend
                    } else {
                        dividend % divisor
                    };
                }
            }
            crate::tcg::op::TcgOpcode::SetCondI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let v1 = temps[*s1 as usize];
                    let v2 = temps[*s2 as usize];
                    let cond = *c as u32;
                    let res = match cond {
                        0 => {
                            if v1 == v2 {
                                1
                            } else {
                                0
                            }
                        }
                        1 => {
                            if v1 != v2 {
                                1
                            } else {
                                0
                            }
                        }
                        2 => {
                            if (v1 as i64) < (v2 as i64) {
                                1
                            } else {
                                0
                            }
                        }
                        3 => {
                            if v1 < v2 {
                                1
                            } else {
                                0
                            }
                        }
                        4 if (v1 as i64) >= (v2 as i64) => 1,
                        4 => 0,
                        5 if v1 >= v2 => 1,
                        5 => 0,
                        _ => 0,
                    };
                    temps[*d as usize] = res;
                }
            }
            crate::tcg::op::TcgOpcode::GetGprI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(r)) =
                    (&op.args[0], &op.args[1])
                {
                    temps[*d as usize] = cpu.read_gpr(*r as u8);
                }
            }
            crate::tcg::op::TcgOpcode::SetGprI64 => {
                if let (crate::tcg::op::TcgArg::Const(r), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    cpu.write_gpr(*r as u8, temps[*s as usize]);
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd8Signed => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u8(addr).unwrap_or(0) as i8 as i64 as u64;
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd8Unsigned => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u8(addr).unwrap_or(0) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd16Signed => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u16(addr).unwrap_or(0) as i16 as i64 as u64;
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd16Unsigned => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u16(addr).unwrap_or(0) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd32Signed => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u32(addr).unwrap_or(0) as i32 as i64 as u64;
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd32Unsigned => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u32(addr).unwrap_or(0) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u64(addr).unwrap_or(0);
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt8 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    cpu.clear_reservation_if_overlap(addr, 1);
                    let _ = mem.write_u8(addr, temps[*s as usize] as u8);
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt16 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    cpu.clear_reservation_if_overlap(addr, 2);
                    let _ = mem.write_u16(addr, temps[*s as usize] as u16);
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt32 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    cpu.clear_reservation_if_overlap(addr, 4);
                    let _ = mem.write_u32(addr, temps[*s as usize] as u32);
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt64 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (&op.args[0], &op.args[1])
                {
                    let addr = temps[*a as usize];
                    cpu.clear_reservation_if_overlap(addr, 8);
                    let _ = mem.write_u64(addr, temps[*s as usize]);
                }
            }
            crate::tcg::op::TcgOpcode::SetNextPcI64 => {
                if let crate::tcg::op::TcgArg::Temp(t) = &op.args[0] {
                    next_pc = Some(temps[*t as usize]);
                } else if let crate::tcg::op::TcgArg::Const(c) = &op.args[0] {
                    next_pc = Some(*c);
                }
            }
            crate::tcg::op::TcgOpcode::SetLabel => {
                // marker, nothing
            }
            crate::tcg::op::TcgOpcode::Br => {
                if let crate::tcg::op::TcgArg::Label(l) = &op.args[0]
                    && (*l as usize) < label_pos.len()
                    && let Some(pos) = label_pos[*l as usize]
                {
                    i = pos;
                    continue;
                }
            }
            crate::tcg::op::TcgOpcode::BrCondI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Const(c),
                    crate::tcg::op::TcgArg::Label(l),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let v1 = temps[*s1 as usize];
                    let v2 = temps[*s2 as usize];
                    let cond = *c as u32;
                    let take = match cond {
                        0 => v1 == v2,
                        1 => v1 != v2,
                        2 => (v1 as i64) < (v2 as i64),
                        3 => v1 < v2,
                        4 => (v1 as i64) >= (v2 as i64),
                        5 => v1 >= v2,
                        _ => false,
                    };
                    if take
                        && (*l as usize) < label_pos.len()
                        && let Some(pos) = label_pos[*l as usize]
                    {
                        i = pos;
                        continue;
                    }
                }
            }
            crate::tcg::op::TcgOpcode::ExitTb => {
                break;
            }
            crate::tcg::op::TcgOpcode::Call => {
                if let (crate::tcg::op::TcgArg::Const(h), _) = (&op.args[0], &op.args[1])
                    && *h == 0
                    && let Ok(Some(code)) = crate::syscall::handle_ecall(cpu, mem)
                {
                    std::process::exit(code);
                }
            }
            crate::tcg::op::TcgOpcode::LrW => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a), _) =
                    (&op.args[0], &op.args[1], &op.args[2])
                {
                    let addr = temps[*a as usize];
                    let val = mem.read_u32(addr).unwrap_or(0) as i32 as u64;
                    temps[*d as usize] = val;
                    cpu.set_reservation(addr, 4);
                }
            }
            crate::tcg::op::TcgOpcode::LrD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a), _) =
                    (&op.args[0], &op.args[1], &op.args[2])
                {
                    let addr = temps[*a as usize];
                    let val = mem.read_u64(addr).unwrap_or(0);
                    temps[*d as usize] = val;
                    cpu.set_reservation(addr, 8);
                }
            }
            crate::tcg::op::TcgOpcode::ScW => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(a),
                    crate::tcg::op::TcgArg::Temp(v),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let addr = temps[*a as usize];
                    let val = temps[*v as usize] as u32;
                    let succ = if cpu.check_and_clear_reservation(addr, 4) {
                        let _ = mem.write_u32(addr, val);
                        0
                    } else {
                        1
                    };
                    temps[*d as usize] = succ;
                }
            }
            crate::tcg::op::TcgOpcode::ScD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(a),
                    crate::tcg::op::TcgArg::Temp(v),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let addr = temps[*a as usize];
                    let val = temps[*v as usize];
                    let succ = if cpu.check_and_clear_reservation(addr, 8) {
                        let _ = mem.write_u64(addr, val);
                        0
                    } else {
                        1
                    };
                    temps[*d as usize] = succ;
                }
            }
            crate::tcg::op::TcgOpcode::GetFprI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(r)) =
                    (&op.args[0], &op.args[1])
                {
                    temps[*d as usize] = cpu.read_fpr(*r as u8);
                }
            }
            crate::tcg::op::TcgOpcode::SetFprI64 => {
                if let (crate::tcg::op::TcgArg::Const(r), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    cpu.write_fpr(*r as u8, temps[*s as usize]);
                }
            }
            crate::tcg::op::TcgOpcode::FAddS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let r = a + b;
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FAddD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let r = a + b;
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FSubS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let r = a - b;
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FSubD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let r = a - b;
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FMulS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let r = a * b;
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FMulD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let r = a * b;
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FDivS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let r = a / b;
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FDivD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let r = a / b;
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FSqrtS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let a = f32::from_bits(temps[*s as usize] as u32);
                    let r = a.sqrt();
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FSqrtD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let a = f64::from_bits(temps[*s as usize]);
                    let r = a.sqrt();
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FMaddS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let c = f32::from_bits(temps[*s3 as usize] as u32);
                    let r = a.mul_add(b, c);
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FMaddD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let c = f64::from_bits(temps[*s3 as usize]);
                    let r = a.mul_add(b, c);
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FMsubS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let c = f32::from_bits(temps[*s3 as usize] as u32);
                    let r = a.mul_add(b, -c);
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FMsubD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let c = f64::from_bits(temps[*s3 as usize]);
                    let r = a.mul_add(b, -c);
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FNmaddS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let c = f32::from_bits(temps[*s3 as usize] as u32);
                    let r = (-a).mul_add(b, -c);
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FNmaddD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let c = f64::from_bits(temps[*s3 as usize]);
                    let r = (-a).mul_add(b, -c);
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FNmsubS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let c = f32::from_bits(temps[*s3 as usize] as u32);
                    let r = (-a).mul_add(b, c);
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FNmsubD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Temp(s3),
                ) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let c = f64::from_bits(temps[*s3 as usize]);
                    let r = (-a).mul_add(b, c);
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FCvtWS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f32::from_bits(temps[*s as usize] as u32);
                    temps[*d as usize] = (f as i32 as i64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtWUS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f32::from_bits(temps[*s as usize] as u32);
                    let u = if f <= 0.0 {
                        0u32
                    } else if f >= (u32::MAX as f32) {
                        u32::MAX
                    } else {
                        f as u32
                    };
                    temps[*d as usize] = u as u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtLS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f32::from_bits(temps[*s as usize] as u32);
                    temps[*d as usize] = (f as i64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtLUS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f32::from_bits(temps[*s as usize] as u32);
                    let u = if f <= 0.0 {
                        0u64
                    } else if f >= (u64::MAX as f32) {
                        u64::MAX
                    } else {
                        f as u64
                    };
                    temps[*d as usize] = u;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtSW => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let i = temps[*s as usize] as i32 as f32;
                    temps[*d as usize] = (i.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtSL => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let i = temps[*s as usize] as i64 as f32;
                    temps[*d as usize] = (i.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtSWU => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let u = temps[*s as usize] as u32 as f32;
                    temps[*d as usize] = (u.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtSLU => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let u = temps[*s as usize] as f32;
                    temps[*d as usize] = (u.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtWD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f64::from_bits(temps[*s as usize]);
                    temps[*d as usize] = (f as i32 as i64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtWUD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f64::from_bits(temps[*s as usize]);
                    let u = if f <= 0.0 {
                        0u32
                    } else if f >= (u32::MAX as f64) {
                        u32::MAX
                    } else {
                        f as u32
                    };
                    temps[*d as usize] = u as u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtLD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f64::from_bits(temps[*s as usize]);
                    temps[*d as usize] = (f as i64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtLUD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let f = f64::from_bits(temps[*s as usize]);
                    let u = if f <= 0.0 {
                        0u64
                    } else if f >= (u64::MAX as f64) {
                        u64::MAX
                    } else {
                        f as u64
                    };
                    temps[*d as usize] = u;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtDW => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let i = temps[*s as usize] as i32 as f64;
                    temps[*d as usize] = i.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FCvtDL => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let i = temps[*s as usize] as i64 as f64;
                    temps[*d as usize] = i.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FCvtDWU => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let u = temps[*s as usize] as u32 as f64;
                    temps[*d as usize] = u.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FCvtDLU => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let u = temps[*s as usize] as f64;
                    temps[*d as usize] = u.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FCvtSD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let dval = f64::from_bits(temps[*s as usize]);
                    let s = dval as f32;
                    temps[*d as usize] = (s.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FCvtDS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let sval = f32::from_bits(temps[*s as usize] as u32);
                    let dval = sval as f64;
                    temps[*d as usize] = dval.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FMvXW => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let bits = temps[*s as usize] as u32;
                    temps[*d as usize] = (bits as i32 as i64) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::FMvWX => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let v = temps[*s as usize] as u32;
                    temps[*d as usize] = (v as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FMvXD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    temps[*d as usize] = temps[*s as usize];
                }
            }
            crate::tcg::op::TcgOpcode::FMvDX => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    temps[*d as usize] = temps[*s as usize];
                }
            }
            crate::tcg::op::TcgOpcode::FSgnjS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let mut a = temps[*s1 as usize] as u32;
                    let b = temps[*s2 as usize] as u32;
                    a = (a & 0x7fff_ffff) | (b & 0x8000_0000);
                    temps[*d as usize] = (a as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FSgnjD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let mut a = temps[*s1 as usize];
                    let b = temps[*s2 as usize];
                    a = (a & 0x7fff_ffff_ffff_ffff) | (b & 0x8000_0000_0000_0000);
                    temps[*d as usize] = a;
                }
            }
            crate::tcg::op::TcgOpcode::FSgnjnS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let mut a = temps[*s1 as usize] as u32;
                    let b = temps[*s2 as usize] as u32;
                    a = (a & 0x7fff_ffff) | ((!b) & 0x8000_0000);
                    temps[*d as usize] = (a as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FSgnjnD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let mut a = temps[*s1 as usize];
                    let b = temps[*s2 as usize];
                    a = (a & 0x7fff_ffff_ffff_ffff) | ((!b) & 0x8000_0000_0000_0000);
                    temps[*d as usize] = a;
                }
            }
            crate::tcg::op::TcgOpcode::FSgnjxS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let mut a = temps[*s1 as usize] as u32;
                    let b = temps[*s2 as usize] as u32;
                    a = (a & 0x7fff_ffff) | ((a ^ b) & 0x8000_0000);
                    temps[*d as usize] = (a as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FSgnjxD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let mut a = temps[*s1 as usize];
                    let b = temps[*s2 as usize];
                    a = (a & 0x7fff_ffff_ffff_ffff) | ((a ^ b) & 0x8000_0000_0000_0000);
                    temps[*d as usize] = a;
                }
            }
            crate::tcg::op::TcgOpcode::FMinS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let r = if a.is_nan() {
                        b
                    } else if b.is_nan() {
                        a
                    } else if a < b {
                        a
                    } else {
                        b
                    };
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FMinD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let r = if a.is_nan() {
                        b
                    } else if b.is_nan() {
                        a
                    } else if a < b {
                        a
                    } else {
                        b
                    };
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FMaxS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    let r = if a.is_nan() {
                        b
                    } else if b.is_nan() {
                        a
                    } else if a > b {
                        a
                    } else {
                        b
                    };
                    temps[*d as usize] = (r.to_bits() as u64) | 0xffff_ffff_0000_0000u64;
                }
            }
            crate::tcg::op::TcgOpcode::FMaxD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    let r = if a.is_nan() {
                        b
                    } else if b.is_nan() {
                        a
                    } else if a > b {
                        a
                    } else {
                        b
                    };
                    temps[*d as usize] = r.to_bits();
                }
            }
            crate::tcg::op::TcgOpcode::FeqS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    temps[*d as usize] = if a == b { 1 } else { 0 };
                }
            }
            crate::tcg::op::TcgOpcode::FeqD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    temps[*d as usize] = if a == b { 1 } else { 0 };
                }
            }
            crate::tcg::op::TcgOpcode::FltS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    temps[*d as usize] = if a < b { 1 } else { 0 };
                }
            }
            crate::tcg::op::TcgOpcode::FltD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    temps[*d as usize] = if a < b { 1 } else { 0 };
                }
            }
            crate::tcg::op::TcgOpcode::FleS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f32::from_bits(temps[*s1 as usize] as u32);
                    let b = f32::from_bits(temps[*s2 as usize] as u32);
                    temps[*d as usize] = if a <= b { 1 } else { 0 };
                }
            }
            crate::tcg::op::TcgOpcode::FleD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (&op.args[0], &op.args[1], &op.args[2])
                {
                    let a = f64::from_bits(temps[*s1 as usize]);
                    let b = f64::from_bits(temps[*s2 as usize]);
                    temps[*d as usize] = if a <= b { 1 } else { 0 };
                }
            }
            crate::tcg::op::TcgOpcode::FClassS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let bits = temps[*s as usize] as u32;
                    let is_neg = (bits >> 31) != 0;
                    let exp = (bits >> 23) & 0xff;
                    let mant = bits & 0x007fffff;
                    let is_sub = exp == 0 && mant != 0;
                    let is_norm = exp != 0 && exp != 0xff;
                    let is_inf = exp == 0xff && mant == 0;
                    let is_nan = exp == 0xff && mant != 0;
                    let is_zero = exp == 0 && mant == 0;
                    let mut cls: u64 = 0;
                    if is_neg && is_inf {
                        cls = 1;
                    } else if is_neg && is_norm {
                        cls = 1 << 1;
                    } else if is_neg && is_sub {
                        cls = 1 << 2;
                    } else if is_neg && is_zero {
                        cls = 1 << 3;
                    } else if !is_neg && is_zero {
                        cls = 1 << 4;
                    } else if !is_neg && is_sub {
                        cls = 1 << 5;
                    } else if !is_neg && is_norm {
                        cls = 1 << 6;
                    } else if !is_neg && is_inf {
                        cls = 1 << 7;
                    } else if is_nan {
                        cls = if (mant & 0x00400000) != 0 {
                            1 << 9
                        } else {
                            1 << 8
                        };
                    }
                    temps[*d as usize] = cls;
                }
            }
            crate::tcg::op::TcgOpcode::FClassD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (&op.args[0], &op.args[1])
                {
                    let bits = temps[*s as usize];
                    let is_neg = (bits >> 63) != 0;
                    let exp = (bits >> 52) & 0x7ff;
                    let mant = bits & 0x000f_ffff_ffff_ffff;
                    let is_sub = exp == 0 && mant != 0;
                    let is_norm = exp != 0 && exp != 0x7ff;
                    let is_inf = exp == 0x7ff && mant == 0;
                    let is_nan = exp == 0x7ff && mant != 0;
                    let is_zero = exp == 0 && mant == 0;
                    let mut cls: u64 = 0;
                    if is_neg && is_inf {
                        cls = 1;
                    } else if is_neg && is_norm {
                        cls = 1 << 1;
                    } else if is_neg && is_sub {
                        cls = 1 << 2;
                    } else if is_neg && is_zero {
                        cls = 1 << 3;
                    } else if !is_neg && is_zero {
                        cls = 1 << 4;
                    } else if !is_neg && is_sub {
                        cls = 1 << 5;
                    } else if !is_neg && is_norm {
                        cls = 1 << 6;
                    } else if !is_neg && is_inf {
                        cls = 1 << 7;
                    } else if is_nan {
                        cls = if (mant & (1u64 << 51)) != 0 {
                            1 << 9
                        } else {
                            1 << 8
                        };
                    }
                    temps[*d as usize] = cls;
                }
            }
        }
        i += 1;
    }
    next_pc
}
