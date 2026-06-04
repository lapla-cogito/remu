use dynasmrt::DynasmApi as _;
use dynasmrt::DynasmLabelApi as _;

pub fn compile(
    ctx: &crate::tcg::context::TcgContext,
) -> anyhow::Result<dynasmrt::ExecutableBuffer> {
    let mut asm = dynasmrt::x64::Assembler::new().map_err(|e| anyhow::anyhow!("{}", e))?;
    let temp_base: i32 = -32;
    dynasmrt::dynasm!(asm
        ; push rbp
        ; mov rbp, rsp
        ; sub rsp, 256
        ; mov [rbp-8], rdi
        ; mov [rbp-16], rsi
        ; mov [rbp-24], rdx
    );
    for op in &ctx.ops {
        match op.opc {
            crate::tcg::op::TcgOpcode::GetGprI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(r)) =
                    (op.args[0], op.args[1])
                {
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
                if let (crate::tcg::op::TcgArg::Const(r), crate::tcg::op::TcgArg::Temp(s)) =
                    (op.args[0], op.args[1])
                {
                    let off = temp_base - (s as i32) * 8;
                    let reg_off = (r as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off]
                        ; mov rdx, [rbp-8]
                        ; mov [rdx + reg_off], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::GetFprI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(r)) =
                    (op.args[0], op.args[1])
                {
                    let off = temp_base - (d as i32) * 8;
                    let reg_off = (r as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp-16]
                        ; mov rax, [rax + reg_off]
                        ; mov [rbp + off], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::SetFprI64 => {
                if let (crate::tcg::op::TcgArg::Const(r), crate::tcg::op::TcgArg::Temp(s)) =
                    (op.args[0], op.args[1])
                {
                    let off = temp_base - (s as i32) * 8;
                    let reg_off = (r as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off]
                        ; mov rdx, [rbp-16]
                        ; mov [rdx + reg_off], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::AddI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; add rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; add rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
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
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; sub rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; sub rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; sub rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::AndI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; and rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; and rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; and rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::OrI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; or rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; or rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; or rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::XorI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; xor rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; xor rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; xor rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd8Signed => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; movsx rax, BYTE [rdx + rax]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd8Unsigned => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; movzx rax, BYTE [rdx + rax]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd16Signed => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; movsx rax, WORD [rdx + rax]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd16Unsigned => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; movzx rax, WORD [rdx + rax]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd32Signed => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; movsxd rax, DWORD [rdx + rax]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd32Unsigned => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; mov eax, DWORD [rdx + rax]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuLd64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; mov rax, [rdx + rax]
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt8 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_s = temp_base - (s as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s]
                        ; mov rdx, [rbp - 24]
                        ; mov rcx, [rbp + off_a]
                        ; mov r11, [rbp - 8]
                        ; mov QWORD [r11 + 528], 0
                        ; mov QWORD [r11 + 536], 0
                        ; mov BYTE [rdx + rcx], al
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt16 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_s = temp_base - (s as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s]
                        ; mov rdx, [rbp - 24]
                        ; mov rcx, [rbp + off_a]
                        ; mov r11, [rbp - 8]
                        ; mov QWORD [r11 + 528], 0
                        ; mov QWORD [r11 + 536], 0
                        ; mov WORD [rdx + rcx], ax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt32 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_s = temp_base - (s as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s]
                        ; mov rdx, [rbp - 24]
                        ; mov rcx, [rbp + off_a]
                        ; mov r11, [rbp - 8]
                        ; mov QWORD [r11 + 528], 0
                        ; mov QWORD [r11 + 536], 0
                        ; mov DWORD [rdx + rcx], eax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::QemuSt64 => {
                if let (crate::tcg::op::TcgArg::Temp(s), crate::tcg::op::TcgArg::Temp(a)) =
                    (op.args[0], op.args[1])
                {
                    let off_s = temp_base - (s as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s]
                        ; mov rdx, [rbp - 24]
                        ; mov rcx, [rbp + off_a]
                        ; mov r11, [rbp - 8]
                        ; mov QWORD [r11 + 528], 0
                        ; mov QWORD [r11 + 536], 0
                        ; mov [rdx + rcx], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::ShlI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, [rbp + off_s2]
                        ; shl rax, cl
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; shl rax, cl
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::SarI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, [rbp + off_s2]
                        ; sar rax, cl
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; sar rax, cl
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::MulI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; imul rax, [rbp + off_s2]
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, QWORD ci
                        ; imul rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                } else if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let c1i = c1 as i64;
                    let c2i = c2 as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD c1i
                        ; mov rcx, QWORD c2i
                        ; imul rax, rcx
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::MulhI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; imul QWORD [rbp + off_s2]
                        ; mov [rbp + off_d], rdx
                    );
                }
            }
            crate::tcg::op::TcgOpcode::MulhsuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rdx, [rbp + off_s2]
                        ; imul rdx
                        ; mov [rbp + off_d], rdx
                    );
                }
            }
            crate::tcg::op::TcgOpcode::MulhuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mul QWORD [rbp + off_s2]
                        ; mov [rbp + off_d], rdx
                    );
                }
            }
            crate::tcg::op::TcgOpcode::DivsI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, [rbp + off_s2]
                        ; cqo
                        ; idiv rcx
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::DivuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, [rbp + off_s2]
                        ; xor rdx, rdx
                        ; div rcx
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::RemsI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, [rbp + off_s2]
                        ; cqo
                        ; idiv rcx
                        ; mov [rbp + off_d], rdx
                    );
                }
            }
            crate::tcg::op::TcgOpcode::RemuI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s1]
                        ; mov rcx, [rbp + off_s2]
                        ; xor rdx, rdx
                        ; div rcx
                        ; mov [rbp + off_d], rdx
                    );
                }
            }
            crate::tcg::op::TcgOpcode::Call => {
                if let (crate::tcg::op::TcgArg::Const(h), _) = (op.args[0], op.args[1])
                    && h == 0
                {
                    let helper = helper_syscall as *const () as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rdi, [rbp-8]
                        ; mov rsi, [rbp-24]
                        ; mov rax, QWORD helper
                        ; call rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::LrW => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a), _) =
                    (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov r10, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; movsxd rax, DWORD [rdx + r10]
                        ; mov [rbp + off_d], rax
                        ; mov rdx, [rbp - 8]
                        ; mov [rdx + 528], r10
                        ; mov QWORD [rdx + 536], 4
                    );
                }
            }
            crate::tcg::op::TcgOpcode::LrD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(a), _) =
                    (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov r10, [rbp + off_a]
                        ; mov rdx, [rbp - 24]
                        ; mov rax, [rdx + r10]
                        ; mov [rbp + off_d], rax
                        ; mov rdx, [rbp - 8]
                        ; mov [rdx + 528], r10
                        ; mov QWORD [rdx + 536], 8
                    );
                }
            }
            crate::tcg::op::TcgOpcode::ScW => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(a),
                    crate::tcg::op::TcgArg::Temp(v),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    let off_v = temp_base - (v as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov r10, [rbp + off_a]
                        ; mov rdx, [rbp - 8]
                        ; mov rax, [rdx + 528]
                        ; cmp r10, rax
                        ; jne >fail
                        ; mov rax, [rbp + off_v]
                        ; mov r11, [rbp - 24]
                        ; mov DWORD [r11 + r10], eax
                        ; mov QWORD [rdx + 528], 0
                        ; mov QWORD [rdx + 536], 0
                        ; xor rax, rax
                        ; mov [rbp + off_d], rax
                        ; jmp >end
                        ; fail:
                        ; mov rax, 1
                        ; mov [rbp + off_d], rax
                        ; end:
                    );
                }
            }
            crate::tcg::op::TcgOpcode::ScD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(a),
                    crate::tcg::op::TcgArg::Temp(v),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_a = temp_base - (a as i32) * 8;
                    let off_v = temp_base - (v as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov r10, [rbp + off_a]
                        ; mov rdx, [rbp - 8]
                        ; mov rax, [rdx + 528]
                        ; cmp r10, rax
                        ; jne >fail
                        ; mov rax, [rbp + off_v]
                        ; mov r11, [rbp - 24]
                        ; mov [r11 + r10], rax
                        ; mov QWORD [rdx + 528], 0
                        ; mov QWORD [rdx + 536], 0
                        ; xor rax, rax
                        ; mov [rbp + off_d], rax
                        ; jmp >end
                        ; fail:
                        ; mov rax, 1
                        ; mov [rbp + off_d], rax
                        ; end:
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FAddS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movss xmm0, [rbp + off_s1]
                        ; addss xmm0, [rbp + off_s2]
                        ; movss [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FAddD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movsd xmm0, [rbp + off_s1]
                        ; addsd xmm0, [rbp + off_s2]
                        ; movsd [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FSubS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movss xmm0, [rbp + off_s1]
                        ; subss xmm0, [rbp + off_s2]
                        ; movss [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FSubD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movsd xmm0, [rbp + off_s1]
                        ; subsd xmm0, [rbp + off_s2]
                        ; movsd [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FMulS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movss xmm0, [rbp + off_s1]
                        ; mulss xmm0, [rbp + off_s2]
                        ; movss [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FMulD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movsd xmm0, [rbp + off_s1]
                        ; mulsd xmm0, [rbp + off_s2]
                        ; movsd [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FDivS => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movss xmm0, [rbp + off_s1]
                        ; divss xmm0, [rbp + off_s2]
                        ; movss [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FDivD => {
                if let (
                    crate::tcg::op::TcgArg::Temp(d),
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                ) = (op.args[0], op.args[1], op.args[2])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s1 = temp_base - (s1 as i32) * 8;
                    let off_s2 = temp_base - (s2 as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movsd xmm0, [rbp + off_s1]
                        ; divsd xmm0, [rbp + off_s2]
                        ; movsd [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FSqrtS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s = temp_base - (s as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movss xmm0, [rbp + off_s]
                        ; sqrtss xmm0, xmm0
                        ; movss [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FSqrtD => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s = temp_base - (s as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movsd xmm0, [rbp + off_s]
                        ; sqrtsd xmm0, xmm0
                        ; movsd [rbp + off_d], xmm0
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FCvtWS => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s = temp_base - (s as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; movss xmm0, [rbp + off_s]
                        ; cvttss2si rax, xmm0
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::FCvtSW => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s = temp_base - (s as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; cvtsi2ss xmm0, [rbp + off_s]
                        ; movss [rbp + off_d], xmm0
                    );
                }
            }
            _ => {}
        }
    }
    dynasmrt::dynasm!(asm
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
