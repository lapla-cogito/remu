pub fn translate_block(
    start_pc: u64,
    mem: &crate::memory::GuestMemory,
    max_insns: u32,
) -> (crate::tcg::context::TcgContext, u64) {
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
            crate::decode::Instr::Addiw { rd, rs1, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_add_i64(t3, t1, t2);
                    // sign extend 32->64 for *w
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Slliw { rd, rs1, shamt } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(shamt as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_shl_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Srliw { rd, rs1, shamt } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(shamt as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_shr_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Sraiw { rd, rs1, shamt } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(shamt as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_sar_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Mulw { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_mul_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Divw { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_divs_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Divuw { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_divu_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Remw { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_rems_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
                }
            }
            crate::decode::Instr::Remuw { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_remu_i64(t3, t1, t2);
                    let t4 = ctx.new_temp();
                    ctx.gen_shl_i64(t4, t3, ctx.new_const(32));
                    let t5 = ctx.new_temp();
                    ctx.gen_sar_i64(t5, t4, ctx.new_const(32));
                    ctx.gen_set_gpr_i64(rd, t5);
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
            crate::decode::Instr::Andi { rd, rs1, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_and_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Ori { rd, rs1, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_or_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Xori { rd, rs1, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_xor_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Slti { rd, rs1, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_set_cond_i64(t3, t1, t2, 2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Sltiu { rd, rs1, imm } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_const(imm as u64);
                    let t3 = ctx.new_temp();
                    ctx.gen_set_cond_i64(t3, t1, t2, 3);
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
            crate::decode::Instr::Mul { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_mul_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Mulh { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_mulh_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Mulhsu { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_mulhsu_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Mulhu { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_mulhu_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Div { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_divs_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Divu { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_divu_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Rem { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_rems_i64(t3, t1, t2);
                    ctx.gen_set_gpr_i64(rd, t3);
                }
            }
            crate::decode::Instr::Remu { rd, rs1, rs2 } => {
                if rd != 0 {
                    let t1 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t1, rs1);
                    let t2 = ctx.new_temp();
                    ctx.gen_get_gpr_i64(t2, rs2);
                    let t3 = ctx.new_temp();
                    ctx.gen_remu_i64(t3, t1, t2);
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
                    ctx.gen_qemu_ld8_signed(td, taddr);
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
                    ctx.gen_qemu_ld16_signed(td, taddr);
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
                    ctx.gen_qemu_ld32_signed(td, taddr);
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
                    ctx.gen_qemu_ld64(td, taddr);
                    ctx.gen_set_gpr_i64(rd, td);
                }
            }
            crate::decode::Instr::Lbu { rd, rs1, imm } => {
                if rd != 0 {
                    let taddr = ctx.new_temp();
                    let trs = ctx.new_temp();
                    ctx.gen_get_gpr_i64(trs, rs1);
                    let timm = ctx.new_const(imm as u64);
                    ctx.gen_add_i64(taddr, trs, timm);
                    let td = ctx.new_temp();
                    ctx.gen_qemu_ld8_unsigned(td, taddr);
                    ctx.gen_set_gpr_i64(rd, td);
                }
            }
            crate::decode::Instr::Lhu { rd, rs1, imm } => {
                if rd != 0 {
                    let taddr = ctx.new_temp();
                    let trs = ctx.new_temp();
                    ctx.gen_get_gpr_i64(trs, rs1);
                    let timm = ctx.new_const(imm as u64);
                    ctx.gen_add_i64(taddr, trs, timm);
                    let td = ctx.new_temp();
                    ctx.gen_qemu_ld16_unsigned(td, taddr);
                    ctx.gen_set_gpr_i64(rd, td);
                }
            }
            crate::decode::Instr::Lwu { rd, rs1, imm } => {
                if rd != 0 {
                    let taddr = ctx.new_temp();
                    let trs = ctx.new_temp();
                    ctx.gen_get_gpr_i64(trs, rs1);
                    let timm = ctx.new_const(imm as u64);
                    ctx.gen_add_i64(taddr, trs, timm);
                    let td = ctx.new_temp();
                    ctx.gen_qemu_ld32_unsigned(td, taddr);
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
                ctx.gen_qemu_st8(trs2, taddr);
            }
            crate::decode::Instr::Sh { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let trs2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs2, rs2);
                ctx.gen_qemu_st16(trs2, taddr);
            }
            crate::decode::Instr::Sw { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let trs2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs2, rs2);
                ctx.gen_qemu_st32(trs2, taddr);
            }
            crate::decode::Instr::Sd { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let trs2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs2, rs2);
                ctx.gen_qemu_st64(trs2, taddr);
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
            crate::decode::Instr::Beq { rs1, rs2, imm } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t2, rs2);
                let l = ctx.new_label();
                ctx.gen_brcond_i64(t1, t2, 0, l);
                let t_fall = ctx.new_const(after_pc);
                ctx.gen_set_next_pc(t_fall);
                ctx.gen_exit_tb();
                ctx.gen_set_label(l);
                let t_taken = ctx.new_const(pc.wrapping_add(imm as u64));
                ctx.gen_set_next_pc(t_taken);
                ctx.gen_exit_tb();
                break;
            }
            crate::decode::Instr::Bne { rs1, rs2, imm } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t2, rs2);
                let l = ctx.new_label();
                ctx.gen_brcond_i64(t1, t2, 1, l);
                let t_fall = ctx.new_const(after_pc);
                ctx.gen_set_next_pc(t_fall);
                ctx.gen_exit_tb();
                ctx.gen_set_label(l);
                let t_taken = ctx.new_const(pc.wrapping_add(imm as u64));
                ctx.gen_set_next_pc(t_taken);
                ctx.gen_exit_tb();
                break;
            }
            crate::decode::Instr::Blt { rs1, rs2, imm } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t2, rs2);
                let l = ctx.new_label();
                ctx.gen_brcond_i64(t1, t2, 2, l);
                let t_fall = ctx.new_const(after_pc);
                ctx.gen_set_next_pc(t_fall);
                ctx.gen_exit_tb();
                ctx.gen_set_label(l);
                let t_taken = ctx.new_const(pc.wrapping_add(imm as u64));
                ctx.gen_set_next_pc(t_taken);
                ctx.gen_exit_tb();
                break;
            }
            crate::decode::Instr::Bge { rs1, rs2, imm } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t2, rs2);
                let l = ctx.new_label();
                ctx.gen_brcond_i64(t1, t2, 4, l);
                let t_fall = ctx.new_const(after_pc);
                ctx.gen_set_next_pc(t_fall);
                ctx.gen_exit_tb();
                ctx.gen_set_label(l);
                let t_taken = ctx.new_const(pc.wrapping_add(imm as u64));
                ctx.gen_set_next_pc(t_taken);
                ctx.gen_exit_tb();
                break;
            }
            crate::decode::Instr::Bltu { rs1, rs2, imm } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t2, rs2);
                let l = ctx.new_label();
                ctx.gen_brcond_i64(t1, t2, 3, l);
                let t_fall = ctx.new_const(after_pc);
                ctx.gen_set_next_pc(t_fall);
                ctx.gen_exit_tb();
                ctx.gen_set_label(l);
                let t_taken = ctx.new_const(pc.wrapping_add(imm as u64));
                ctx.gen_set_next_pc(t_taken);
                ctx.gen_exit_tb();
                break;
            }
            crate::decode::Instr::Bgeu { rs1, rs2, imm } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t2, rs2);
                let l = ctx.new_label();
                ctx.gen_brcond_i64(t1, t2, 5, l);
                let t_fall = ctx.new_const(after_pc);
                ctx.gen_set_next_pc(t_fall);
                ctx.gen_exit_tb();
                ctx.gen_set_label(l);
                let t_taken = ctx.new_const(pc.wrapping_add(imm as u64));
                ctx.gen_set_next_pc(t_taken);
                ctx.gen_exit_tb();
                break;
            }
            crate::decode::Instr::Flw { rd, rs1, imm } => {
                let taddr = ctx.new_temp();
                let trs = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs, timm);
                let td = ctx.new_temp();
                ctx.gen_qemu_ld32_unsigned(td, taddr);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::Fld { rd, rs1, imm } => {
                let taddr = ctx.new_temp();
                let trs = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs, timm);
                let td = ctx.new_temp();
                ctx.gen_qemu_ld64(td, taddr);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::Fsw { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let tv = ctx.new_temp();
                ctx.gen_get_fpr_i64(tv, rs2);
                ctx.gen_qemu_st32(tv, taddr);
            }
            crate::decode::Instr::Fsd { rs1, rs2, imm } => {
                let taddr = ctx.new_temp();
                let trs1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(trs1, rs1);
                let timm = ctx.new_const(imm as u64);
                ctx.gen_add_i64(taddr, trs1, timm);
                let tv = ctx.new_temp();
                ctx.gen_get_fpr_i64(tv, rs2);
                ctx.gen_qemu_st64(tv, taddr);
            }
            crate::decode::Instr::FAddS {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fadd_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FAddD {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fadd_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FSubS {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsub_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FSubD {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsub_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FMulS {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fmul_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FMulD {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fmul_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FDivS {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fdiv_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FDivD {
                rd,
                rs1,
                rs2,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fdiv_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FSqrtS { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fsqrt_s(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FSqrtD { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fsqrt_d(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FMaddS {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fmadd_s(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FMaddD {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fmadd_d(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FMsubS {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fmsub_s(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FMsubD {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fmsub_d(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FNmaddS {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fnmadd_s(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FNmaddD {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fnmadd_d(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FNmsubS {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fnmsub_s(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FNmsubD {
                rd,
                rs1,
                rs2,
                rs3,
                _rm: _,
            } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let t3 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t3, rs3);
                let td = ctx.new_temp();
                ctx.gen_fnmsub_d(td, t1, t2, t3);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtWS { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_ws(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtWUS { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_wus(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtLS { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_ls(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtLUS { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_lus(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtSW { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_sw(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtSL { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_sl(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtSWU { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_swu(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtSLU { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_slu(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtWD { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_wd(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtWUD { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_wud(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtLD { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_ld(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtLUD { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_lud(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtDW { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_dw(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtDL { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_dl(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtDWU { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_dwu(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtDLU { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_dlu(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtSD { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_sd(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FcvtDS { rd, rs1, _rm: _ } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fcvt_ds(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FmvXW { rd, rs1 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fmv_xw(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FmvWX { rd, rs1 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fmv_wx(td, t1);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FmvXD { rd, rs1 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                ctx.gen_set_gpr_i64(rd, t1);
            }
            crate::decode::Instr::FmvDX { rd, rs1 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_gpr_i64(t1, rs1);
                ctx.gen_set_fpr_i64(rd, t1);
            }
            crate::decode::Instr::FsgnjS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsgnj_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FsgnjD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsgnj_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FsgnjnS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsgnjn_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FsgnjnD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsgnjn_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FsgnjxS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsgnjx_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FsgnjxD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fsgnjx_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FminS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fmin_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FminD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fmin_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FmaxS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fmax_s(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FmaxD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fmax_d(td, t1, t2);
                ctx.gen_set_fpr_i64(rd, td);
            }
            crate::decode::Instr::FeqS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_feq_s(td, t1, t2);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FeqD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_feq_d(td, t1, t2);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FltS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_flt_s(td, t1, t2);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FltD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_flt_d(td, t1, t2);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FleS { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fle_s(td, t1, t2);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FleD { rd, rs1, rs2 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let t2 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t2, rs2);
                let td = ctx.new_temp();
                ctx.gen_fle_d(td, t1, t2);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FclassS { rd, rs1 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fclass_s(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::FclassD { rd, rs1 } => {
                let t1 = ctx.new_temp();
                ctx.gen_get_fpr_i64(t1, rs1);
                let td = ctx.new_temp();
                ctx.gen_fclass_d(td, t1);
                ctx.gen_set_gpr_i64(rd, td);
            }
            crate::decode::Instr::Unknown(_) => {
                break;
            }
        }
        pc = after_pc;
    }
    (ctx, pc)
}
