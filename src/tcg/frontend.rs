pub fn translate_block(start_pc: u64, mem: &crate::memory::GuestMemory, max_insns: u32) -> (crate::tcg::context::TcgContext, u64) {
    let mut ctx = crate::tcg::context::TcgContext::new();
    let mut pc = start_pc;
    let mut count: u32 = 0;
    loop {
        if count >= max_insns {
            break;
        }
        let (ilen, instr) = match crate::decode::fetch_decode(mem, pc) {
            Ok(v) => v,
            Err(_) => break,
        };
        count += 1;
        let after_pc = pc.wrapping_add(ilen as u64);
        match instr {
            crate::decode::Instr::Addi { rd, rs1, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_add_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Add { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_add_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Sub { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_sub_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::And { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_and_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Or { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_or_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Xor { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_xor_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Sll { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_shl_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Srl { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_shr_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Sra { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_sar_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Slt { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_set_cond_i64(t3, t1, t2, 2); // 2: signed lt
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Sltu { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_set_cond_i64(t3, t1, t2, 3); // 3: unsigned lt
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Slli { rd, rs1, shamt } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(shamt as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_shl_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Srli { rd, rs1, shamt } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(shamt as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_shr_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Srai { rd, rs1, shamt } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(shamt as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_sar_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Auipc { rd, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_const(pc);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_add_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Lui { rd, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_const(imm as u64);
                    ctx.gen_set_gpr_i64(rd, t1);
                }
            }
            crate::decode::Instr::Lb { rd, rs1, imm } => {
                if rd != 0 {
                    let taddr = ctx.new_temp();
                    let trs = ctx.new_temp();
                    ctx.gen_get_gpr_i64(trs, rs1);
                    let timm = ctx.new_const(imm as u64);
                    ctx.gen_add_i64(taddr, trs, timm);
                    let td = ctx.new_temp();
                    ctx.gen_qemu_ld_i64(td, taddr);
                    // for byte, we need sign extend, but for now full 64 assume, later fix
                    ctx.gen_set_gpr_i64(rd, td);
                }
            }
            crate::decode::Instr::Lh { rd, rs1, imm } => {
                if rd != 0 {
                    let taddr = ctx.new_temp();
                    let trs = ctx.new_temp();
                    ctx.gen_get_gpr_i64(trs, rs1);
                    let timm = ctx.new_const(imm as u64);
                    ctx.gen_add_i64(taddr, trs, timm);
                    let td = ctx.new_temp();
                    ctx.gen_qemu_ld_i64(td, taddr);
                    ctx.gen_set_gpr_i64(rd, td);
                }
            }
            crate::decode::Instr::Lw { rd, rs1, imm } => {
                if rd != 0 {
                    let taddr = ctx.new_temp();
                    let trs = ctx.new_temp();
                    ctx.gen_get_gpr_i64(trs, rs1);
                    let timm = ctx.new_const(imm as u64);
                    ctx.gen_add_i64(taddr, trs, timm);
                    let td = ctx.new_temp();
                    ctx.gen_qemu_ld_i64(td, taddr);
                    ctx.gen_set_gpr_i64(rd, td);
                }
            }
            crate::decode::Instr::Ld { rd, rs1, imm } => {
                if rd != 0 {
                    let taddr = ctx.new_temp();
                    let trs = ctx.new_temp();
                    ctx.gen_get_gpr_i64(trs, rs1);
                    let timm = ctx.new_const(imm as u64);
                    ctx.gen_add_i64(taddr, trs, timm);
                    let td = ctx.new_temp();
                    ctx.gen_qemu_ld_i64(td, taddr);
                    ctx.gen_set_gpr_i64(rd, td);
                }
            }
            crate::decode::Instr::Sb { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let trs2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs2, rs2);
                ctx.gen_qemu_st_i64(trs2, taddr);
            }
            crate::decode::Instr::Sh { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let trs2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs2, rs2);
                ctx.gen_qemu_st_i64(trs2, taddr);
            }
            crate::decode::Instr::Sw { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let trs2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs2, rs2);
                ctx.gen_qemu_st_i64(trs2, taddr);
            }
            crate::decode::Instr::Sd { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let trs2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs2, rs2);
                ctx.gen_qemu_st_i64(trs2, taddr);
            }
            crate::decode::Instr::Jal { rd, imm: _ } => {
                if rd != 0 {
                    let tlink = ctx.new_const(after_pc);
                    ctx.gen_set_gpr_i64(rd, tlink);
                }
                break;
            }
            crate::decode::Instr::Jalr { rd, rs1: _, imm: _ } => {
                if rd != 0 {
                    let tlink = ctx.new_const(after_pc);
                    ctx.gen_set_gpr_i64(rd, tlink);
                }
                break;
            }
            crate::decode::Instr::Ecall => {
                ctx.gen_call(0, 0);
                pc = after_pc;
                break;
            }
            crate::decode::Instr::Beq { .. } | crate::decode::Instr::Bne { .. } | crate::decode::Instr::Blt { .. } | crate::decode::Instr::Bge { .. } | crate::decode::Instr::Bltu { .. } | crate::decode::Instr::Bgeu { .. } => {
                break;
            }
            crate::decode::Instr::Unknown(_) => {
                break;
            }
        }
        pc = after_pc;
    }
    (ctx, pc)
}
