pub fn execute_tcg(ctx: &crate::tcg::context::TcgContext, cpu: &mut crate::cpu::Cpu, mem: &mut crate::memory::GuestMemory) -> Option<u64> {
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
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) = (&op.args[0], &op.args[1]) {
                    temps[*d as usize] = temps[*s as usize];
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c)) = (&op.args[0], &op.args[1]) {
                    temps[*d as usize] = *c;
                }
            }
            crate::tcg::op::TcgOpcode::AddI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize].wrapping_add(temps[*s2 as usize]);
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Const(c)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize].wrapping_add(*c);
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = *c1 + temps[*s2 as usize];
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c1), crate::tcg::op::TcgArg::Const(c2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = *c1 + *c2;
                }
            }
            crate::tcg::op::TcgOpcode::SubI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize].wrapping_sub(temps[*s2 as usize]);
                }
            }
            crate::tcg::op::TcgOpcode::AndI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize] & temps[*s2 as usize];
                }
            }
            crate::tcg::op::TcgOpcode::OrI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize] | temps[*s2 as usize];
                }
            }
            crate::tcg::op::TcgOpcode::XorI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize] ^ temps[*s2 as usize];
                }
            }
            crate::tcg::op::TcgOpcode::ShlI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize] << (temps[*s2 as usize] & 63);
                }
            }
            crate::tcg::op::TcgOpcode::ShrI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = temps[*s1 as usize] >> (temps[*s2 as usize] & 63);
                }
            }
            crate::tcg::op::TcgOpcode::SarI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (&op.args[0], &op.args[1], &op.args[2]) {
                    temps[*d as usize] = (temps[*s1 as usize] as i64 >> (temps[*s2 as usize] & 63)) as u64;
                }
            }
            crate::tcg::op::TcgOpcode::SetCondI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2), crate::tcg::op::TcgArg::Const(c)) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3]) {
                    let v1 = temps[*s1 as usize];
                    let v2 = temps[*s2 as usize];
                    let cond = *c as u32;
                    let res = match cond {
                        0 => if v1 == v2 { 1 } else { 0 },
                        1 => if v1 != v2 { 1 } else { 0 },
                        2 => if (v1 as i64) < (v2 as i64) { 1 } else { 0 },
                        3 if v1 < v2 => 1,
                        _ => 0,
                    };
                    temps[*d as usize] = res;
                }
            }
            crate::tcg::op::TcgOpcode::GetGprI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(r)) = (&op.args[0], &op.args[1]) {
                    temps[*d as usize] = cpu.read_gpr(*r as u8);
                }
            }
            crate::tcg::op::TcgOpcode::SetGprI64 => {
                if let (crate::tcg::op::TcgArg::Const(r), crate::tcg::op::TcgArg::Temp(s)) = (&op.args[0], &op.args[1]) {
                    cpu.write_gpr(*r as u8, temps[*s as usize]);
                }
            }
            crate::tcg::op::TcgOpcode::QemuLdI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) = (&op.args[0], &op.args[1]) {
                    let addr = temps[*a as usize];
                    temps[*d as usize] = mem.read_u64(addr).unwrap_or(0);
                }
            }
            crate::tcg::op::TcgOpcode::QemuStI64 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) = (&op.args[0], &op.args[1]) {
                    let addr = temps[*a as usize];
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
                if let (crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2), crate::tcg::op::TcgArg::Const(c), crate::tcg::op::TcgArg::Label(l)) = (&op.args[0], &op.args[1], &op.args[2], &op.args[3]) {
                    let v1 = temps[*s1 as usize];
                    let v2 = temps[*s2 as usize];
                    let cond = *c as u32;
                    let take = match cond {
                        0 => v1 == v2,
                        1 => v1 != v2,
                        2 => (v1 as i64) < (v2 as i64),
                        3 => v1 < v2,
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
        }
        i += 1;
    }
    next_pc
}
