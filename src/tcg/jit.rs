use dynasmrt::DynasmApi as _;

pub fn compile(ctx: &crate::tcg::context::TcgContext) -> anyhow::Result<dynasmrt::ExecutableBuffer> {
    let mut asm = dynasmrt::x64::Assembler::new().map_err(|e| anyhow::anyhow!("{}", e))?;
    let temp_base: i32 = -32;
    dynasmrt::dynasm!(asm
        ; push rbp
        ; mov rbp, rsp
        ; sub rsp, 256
        ; mov [rbp-8], rdi
        ; mov [rbp-16], rsi
    );
    for op in &ctx.ops {
        match op.opc {
            crate::tcg::op::TcgOpcode::GetGprI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(r)) = (op.args[0], op.args[1]) {
                    let off = temp_base - (d as i32) * 8;
                    let reg_off = (r as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp-8]
                        ; mov rax, [rax + reg_off]
                        ; mov [rbp + off], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::SetGprI64 => {
                if let (crate::tcg::op::TcgArg::Const(r), crate::tcg::op::TcgArg::Temp(s)) = (op.args[0], op.args[1]) {
                    let off = temp_base - (s as i32) * 8;
                    let reg_off = (r as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off]
                        ; mov rdx, [rbp-8]
                        ; mov [rdx + reg_off], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::AddI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Const(c)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; add rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c1), crate::tcg::op::TcgArg::Const(c2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; add rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; add rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::SubI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Const(c)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; sub rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c1), crate::tcg::op::TcgArg::Const(c2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; sub rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; sub rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::AndI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Const(c)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; and rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c1), crate::tcg::op::TcgArg::Const(c2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; and rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; and rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::OrI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Const(c)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; or rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c1), crate::tcg::op::TcgArg::Const(c2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; or rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; or rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::XorI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Const(c)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; xor rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c1), crate::tcg::op::TcgArg::Const(c2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; xor rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s1), crate::tcg::op::TcgArg::Temp(s2)) = (op.args[0], op.args[1], op.args[2]) {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    ::dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; xor rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::Call => {
                if let (crate::tcg::op::TcgArg::Const(h), _) = (op.args[0], op.args[1])
                    && h == 0
                {
                    let helper = helper_syscall as *const () as i64;
                    ::dynasmrt::dynasm!(asm
                        ; mov rdi, [rbp-8]
                        ; mov rsi, [rbp-16]
                        ; mov rax, QWORD helper
                        ; call rax
                    );
                }
            }
            _ => {}
        }
    }
    ::dynasmrt::dynasm!(asm
        ; mov rsp, rbp
        ; pop rbp
        ; ret
    );
    let buf = asm.finalize().map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(buf)
}

pub extern "C" fn helper_syscall(gpr: *mut u64, mem: *mut u8) {
    unsafe {
        let a7 = *gpr.add(17);
        if a7 == 64 {
            let fd = *gpr.add(10) as i32;
            let buf = *gpr.add(11) as usize;
            let len = *gpr.add(12) as usize;
            if fd == 1 {
                let slice = std::slice::from_raw_parts(mem.add(buf), len);
                let _ = std::io::Write::write_all(&mut std::io::stdout(), slice);
                *gpr.add(10) = len as u64;
            }
        } else if a7 == 93 {
            let code = *gpr.add(10) as i32;
            std::process::exit(code);
        }
    }
}
