use dynasmrt::DynasmApi as _;
use dynasmrt::DynasmLabelApi as _;
use std::io::Write as _;

pub fn compile(
    ctx: &crate::tcg::context::TcgContext,
    default_next_pc: u64,
    trace: bool,
) -> anyhow::Result<dynasmrt::ExecutableBuffer> {
    let mut asm = dynasmrt::x64::Assembler::new().map_err(|e| anyhow::anyhow!("{}", e))?;
    let temp_base: i32 = -32;
    let nlabels = ctx.num_labels() as usize;
    let mut tcg_labels: Vec<dynasmrt::DynamicLabel> = Vec::with_capacity(nlabels.max(1));
    for _ in 0..nlabels.max(1) {
        tcg_labels.push(asm.new_dynamic_label());
    }
    let epilogue = asm.new_dynamic_label();
    let nextpc_off: i32 = -960;
    let defi = default_next_pc as i64;
    dynasmrt::dynasm!(asm
        ; push rbp
        ; mov rbp, rsp
        ; sub rsp, 1280
        ; mov [rbp-8], rdi
        ; mov [rbp-16], rsi
        ; mov [rbp-24], rdx
        ; mov rax, QWORD defi
        ; mov [rbp + nextpc_off], rax
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
                } else if let (crate::tcg::op::TcgArg::Const(r), crate::tcg::op::TcgArg::Const(c)) =
                    (op.args[0], op.args[1])
                {
                    let reg_off = (r as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD ci
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
            crate::tcg::op::TcgOpcode::MovI64 => {
                if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Temp(s)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let off_s = temp_base - (s as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + off_s]
                        ; mov [rbp + off_d], rax
                    );
                } else if let (crate::tcg::op::TcgArg::Temp(d), crate::tcg::op::TcgArg::Const(c)) =
                    (op.args[0], op.args[1])
                {
                    let off_d = temp_base - (d as i32) * 8;
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD ci
                        ; mov [rbp + off_d], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::Br => {
                if let crate::tcg::op::TcgArg::Label(l) = op.args[0]
                    && (l as usize) < tcg_labels.len()
                {
                    let dl = tcg_labels[l as usize];
                    dynasmrt::dynasm!(asm ; jmp =>dl);
                }
            }
            crate::tcg::op::TcgOpcode::BrCondI64 => {
                if let (
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Const(c),
                    crate::tcg::op::TcgArg::Label(l),
                ) = (op.args[0], op.args[1], op.args[2], op.args[3])
                    && (l as usize) < tcg_labels.len()
                {
                    let off1 = temp_base - (s1 as i32) * 8;
                    let off2 = temp_base - (s2 as i32) * 8;
                    let dl = tcg_labels[l as usize];
                    let cond = c as u32;
                    match cond {
                        0 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; cmp rax, [rbp + off2]
                                ; je =>dl
                            );
                        }
                        1 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; cmp rax, [rbp + off2]
                                ; jne =>dl
                            );
                        }
                        2 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; cmp rax, [rbp + off2]
                                ; jl =>dl
                            );
                        }
                        3 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; cmp rax, [rbp + off2]
                                ; jb =>dl
                            );
                        }
                        4 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; cmp rax, [rbp + off2]
                                ; jge =>dl
                            );
                        }
                        5 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; cmp rax, [rbp + off2]
                                ; jae =>dl
                            );
                        }
                        _ => {}
                    }
                } else if let (
                    crate::tcg::op::TcgArg::Temp(s1),
                    crate::tcg::op::TcgArg::Const(c2),
                    crate::tcg::op::TcgArg::Const(c),
                    crate::tcg::op::TcgArg::Label(l),
                ) = (op.args[0], op.args[1], op.args[2], op.args[3])
                    && (l as usize) < tcg_labels.len()
                {
                    let off1 = temp_base - (s1 as i32) * 8;
                    let ci2 = c2 as i64;
                    let dl = tcg_labels[l as usize];
                    let cond = c as u32;
                    match cond {
                        0 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; je =>dl
                            );
                        }
                        1 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jne =>dl
                            );
                        }
                        2 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jl =>dl
                            );
                        }
                        3 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jb =>dl
                            );
                        }
                        4 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jge =>dl
                            );
                        }
                        5 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, [rbp + off1]
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jae =>dl
                            );
                        }
                        _ => {}
                    }
                } else if let (
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Temp(s2),
                    crate::tcg::op::TcgArg::Const(c),
                    crate::tcg::op::TcgArg::Label(l),
                ) = (op.args[0], op.args[1], op.args[2], op.args[3])
                    && (l as usize) < tcg_labels.len()
                {
                    let off2 = temp_base - (s2 as i32) * 8;
                    let ci1 = c1 as i64;
                    let dl = tcg_labels[l as usize];
                    let cond = c as u32;
                    match cond {
                        0 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; cmp rax, [rbp + off2]
                                ; je =>dl
                            );
                        }
                        1 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; cmp rax, [rbp + off2]
                                ; jne =>dl
                            );
                        }
                        2 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; cmp rax, [rbp + off2]
                                ; jl =>dl
                            );
                        }
                        3 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; cmp rax, [rbp + off2]
                                ; jb =>dl
                            );
                        }
                        4 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; cmp rax, [rbp + off2]
                                ; jge =>dl
                            );
                        }
                        5 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; cmp rax, [rbp + off2]
                                ; jae =>dl
                            );
                        }
                        _ => {}
                    }
                } else if let (
                    crate::tcg::op::TcgArg::Const(c1),
                    crate::tcg::op::TcgArg::Const(c2),
                    crate::tcg::op::TcgArg::Const(c),
                    crate::tcg::op::TcgArg::Label(l),
                ) = (op.args[0], op.args[1], op.args[2], op.args[3])
                    && (l as usize) < tcg_labels.len()
                {
                    let ci1 = c1 as i64;
                    let ci2 = c2 as i64;
                    let dl = tcg_labels[l as usize];
                    let cond = c as u32;
                    match cond {
                        0 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; je =>dl
                            );
                        }
                        1 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jne =>dl
                            );
                        }
                        2 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jl =>dl
                            );
                        }
                        3 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jb =>dl
                            );
                        }
                        4 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jge =>dl
                            );
                        }
                        5 => {
                            dynasmrt::dynasm!(asm
                                ; mov rax, QWORD ci1
                                ; mov rcx, QWORD ci2
                                ; cmp rax, rcx
                                ; jae =>dl
                            );
                        }
                        _ => {}
                    }
                }
            }
            crate::tcg::op::TcgOpcode::SetLabel => {
                if let crate::tcg::op::TcgArg::Label(l) = op.args[0]
                    && (l as usize) < tcg_labels.len()
                {
                    let dl = tcg_labels[l as usize];
                    asm.dynamic_label(dl);
                }
            }
            crate::tcg::op::TcgOpcode::SetNextPcI64 => {
                let off = nextpc_off;
                if let crate::tcg::op::TcgArg::Temp(t) = op.args[0] {
                    let offt = temp_base - (t as i32) * 8;
                    dynasmrt::dynasm!(asm
                        ; mov rax, [rbp + offt]
                        ; mov [rbp + off], rax
                    );
                } else if let crate::tcg::op::TcgArg::Const(c) = op.args[0] {
                    let ci = c as i64;
                    dynasmrt::dynasm!(asm
                        ; mov rax, QWORD ci
                        ; mov [rbp + off], rax
                    );
                }
            }
            crate::tcg::op::TcgOpcode::ExitTb => {
                let el = epilogue;
                dynasmrt::dynasm!(asm ; jmp =>el);
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
    asm.dynamic_label(epilogue);
    dynasmrt::dynasm!(asm
        ; mov rax, [rbp + nextpc_off]
        ; mov rsp, rbp
        ; pop rbp
        ; ret
    );
    let buf = asm.finalize().map_err(|e| anyhow::anyhow!("{:?}", e))?;
    if trace {
        let bytes: &[u8] = &buf;
        println!("out_asm: {} bytes", bytes.len());
        for (i, chunk) in bytes.chunks(16).take(4).enumerate() {
            print!("{:04x}:", i * 16);
            for b in chunk {
                print!(" {:02x}", b);
            }
            println!();
        }
    }
    Ok(buf)
}

pub extern "C" fn helper_syscall(gpr: *mut u64, mem: *mut u8) {
    unsafe {
        let a7 = *gpr.add(17);
        if a7 == 64 {
            let fd = *gpr.add(10) as i32;
            let buf = *gpr.add(11) as usize;
            let len = *gpr.add(12) as usize;
            if fd == 1 || fd == 2 {
                let slice = std::slice::from_raw_parts(mem.add(buf), len);
                let _ = std::io::stdout().write_all(slice);
                *gpr.add(10) = len as u64;
            } else {
                *gpr.add(10) = u64::MAX;
            }
        } else if a7 == 93 || a7 == 94 {
            let code = *gpr.add(10) as i32;
            ::std::process::exit(code);
        } else if a7 == 63 {
            let fd = *gpr.add(10) as i32;
            if fd == 0 {
                *gpr.add(10) = 0;
            } else {
                *gpr.add(10) = (-9i64) as u64;
            }
        } else if a7 == 80 {
            let fd = *gpr.add(10) as i32;
            let statbuf = *gpr.add(11) as usize;
            if fd == 0 || fd == 1 || fd == 2 {
                let mut st = [0u8; 128];
                let mode: u32 = 0x2000 | 0o666;
                let blksize: i32 = 1024;
                st[8..16].copy_from_slice(&1u64.to_le_bytes());
                st[16..20].copy_from_slice(&mode.to_le_bytes());
                st[20..24].copy_from_slice(&1u32.to_le_bytes());
                st[32..40].copy_from_slice(&0u64.to_le_bytes());
                st[56..60].copy_from_slice(&blksize.to_le_bytes());
                let dst = ::std::slice::from_raw_parts_mut(mem.add(statbuf), 128);
                dst.copy_from_slice(&st);
                *gpr.add(10) = 0;
            } else {
                *gpr.add(10) = (-9i64) as u64;
            }
        } else if a7 == 160 {
            let buf = *gpr.add(10) as usize;
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
            let dst = ::std::slice::from_raw_parts_mut(mem.add(buf), uts.len());
            dst.copy_from_slice(&uts);
            *gpr.add(10) = 0;
        } else if a7 == 214 {
            let addr = *gpr.add(10);
            let brk_ptr = (gpr as *mut u8).add(544) as *mut u64;
            if addr == 0 {
                *gpr.add(10) = *brk_ptr;
            } else if addr <= (1u64 << 28) {
                *brk_ptr = addr;
                *gpr.add(10) = addr;
            } else {
                *gpr.add(10) = (-12i64) as u64;
            }
        } else if a7 == 29 {
            let fd = *gpr.add(10) as i32;
            let cmd = *gpr.add(11);
            let arg = *gpr.add(12) as usize;
            if fd == 0 || fd == 1 || fd == 2 {
                if cmd == 0x5401 && arg != 0 {
                    let t = [0u8; 60];
                    let dst = ::std::slice::from_raw_parts_mut(mem.add(arg), 60);
                    dst.copy_from_slice(&t);
                }
                *gpr.add(10) = 0;
            } else {
                *gpr.add(10) = (-25i64) as u64;
            }
        } else if a7 == 56 {
            *gpr.add(10) = (-2i64) as u64;
        } else if a7 == 57 {
            let fd = *gpr.add(10) as i32;
            if fd >= 0 {
                *gpr.add(10) = 0;
            } else {
                *gpr.add(10) = (-9i64) as u64;
            }
        } else if a7 == 66 {
            let fd = *gpr.add(10) as i32;
            let iovp = *gpr.add(11) as usize;
            let iovcnt = *gpr.add(12) as usize;
            if fd == 1 || fd == 2 {
                let mut total: u64 = 0;
                for i in 0..iovcnt {
                    let base_off = iovp + i * 16;
                    let base = {
                        let p = mem.add(base_off);
                        let b0 = *p as u64;
                        let b1 = *p.add(1) as u64;
                        let b2 = *p.add(2) as u64;
                        let b3 = *p.add(3) as u64;
                        let b4 = *p.add(4) as u64;
                        let b5 = *p.add(5) as u64;
                        let b6 = *p.add(6) as u64;
                        let b7 = *p.add(7) as u64;
                        b0 | (b1 << 8)
                            | (b2 << 16)
                            | (b3 << 24)
                            | (b4 << 32)
                            | (b5 << 40)
                            | (b6 << 48)
                            | (b7 << 56)
                    };
                    let len_off = base_off + 8;
                    let len = {
                        let p = mem.add(len_off);
                        let b0 = *p as u64;
                        let b1 = *p.add(1) as u64;
                        let b2 = *p.add(2) as u64;
                        let b3 = *p.add(3) as u64;
                        let b4 = *p.add(4) as u64;
                        let b5 = *p.add(5) as u64;
                        let b6 = *p.add(6) as u64;
                        let b7 = *p.add(7) as u64;
                        (b0 | (b1 << 8)
                            | (b2 << 16)
                            | (b3 << 24)
                            | (b4 << 32)
                            | (b5 << 40)
                            | (b6 << 48)
                            | (b7 << 56)) as usize
                    };
                    if len > 0 {
                        let data = std::slice::from_raw_parts(mem.add(base as usize), len);
                        let _ = std::io::stdout().write_all(data);
                        total += len as u64;
                    }
                }
                *gpr.add(10) = total;
            } else {
                *gpr.add(10) = u64::MAX;
            }
        } else if a7 == 222 {
            let addr = *gpr.add(10);
            let len = *gpr.add(11);
            let mut ret = addr;
            if ret == 0 {
                ret = 0x30000000u64;
            }
            if len > (1u64 << 28) {
                ret = (-12i64) as u64;
            }
            *gpr.add(10) = ret;
        } else {
            *gpr.add(10) = (-38i64) as u64;
        }
    }
}
