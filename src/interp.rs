pub fn step(cpu: &mut crate::cpu::Cpu, mem: &mut crate::memory::GuestMemory) -> anyhow::Result<()> {
    let (ilen, instr) = crate::decode::fetch_decode(mem, cpu.pc)?;
    let npc = cpu.pc.wrapping_add(ilen as u64);
    match instr {
        crate::decode::Instr::Addi { rd, rs1, imm } => {
            let v = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Addiw { rd, rs1, imm } => {
            let v = (cpu.read_gpr(rs1) as i32).wrapping_add(imm as i32) as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Slliw { rd, rs1, shamt } => {
            let v = ((cpu.read_gpr(rs1) as i32 as u32) << shamt) as i32 as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Srliw { rd, rs1, shamt } => {
            let v = ((cpu.read_gpr(rs1) as i32 as u32) >> shamt) as i32 as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Sraiw { rd, rs1, shamt } => {
            let v = ((cpu.read_gpr(rs1) as i32) >> shamt) as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Mulw { rd, rs1, rs2 } => {
            let v =
                ((cpu.read_gpr(rs1) as i32).wrapping_mul(cpu.read_gpr(rs2) as i32)) as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Divw { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1) as i32;
            let divisor = cpu.read_gpr(rs2) as i32;
            let v = if divisor == 0 {
                -1i32 as u32 as u64
            } else if dividend == i32::MIN && divisor == -1 {
                i32::MIN as u64
            } else {
                (dividend / divisor) as i64 as u64
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Divuw { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1) as u32;
            let divisor = cpu.read_gpr(rs2) as u32;
            let v = dividend.checked_div(divisor).unwrap_or(u32::MAX) as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Remw { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1) as i32;
            let divisor = cpu.read_gpr(rs2) as i32;
            let v = if divisor == 0 {
                dividend as u64
            } else if dividend == i32::MIN && divisor == -1 {
                0
            } else {
                (dividend % divisor) as i64 as u64
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Remuw { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1) as u32;
            let divisor = cpu.read_gpr(rs2) as u32;
            let v = if divisor == 0 {
                dividend as u64
            } else {
                (dividend % divisor) as u64
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Add { rd, rs1, rs2 } => {
            let v = cpu.read_gpr(rs1).wrapping_add(cpu.read_gpr(rs2));
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Sub { rd, rs1, rs2 } => {
            let v = cpu.read_gpr(rs1).wrapping_sub(cpu.read_gpr(rs2));
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::And { rd, rs1, rs2 } => {
            let v = cpu.read_gpr(rs1) & cpu.read_gpr(rs2);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Andi { rd, rs1, imm } => {
            let v = cpu.read_gpr(rs1) & (imm as u64);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Ori { rd, rs1, imm } => {
            let v = cpu.read_gpr(rs1) | (imm as u64);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Xori { rd, rs1, imm } => {
            let v = cpu.read_gpr(rs1) ^ (imm as u64);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Slti { rd, rs1, imm } => {
            let v = if (cpu.read_gpr(rs1) as i64) < imm {
                1
            } else {
                0
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Sltiu { rd, rs1, imm } => {
            let v = if cpu.read_gpr(rs1) < (imm as u64) {
                1
            } else {
                0
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Or { rd, rs1, rs2 } => {
            let v = cpu.read_gpr(rs1) | cpu.read_gpr(rs2);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Xor { rd, rs1, rs2 } => {
            let v = cpu.read_gpr(rs1) ^ cpu.read_gpr(rs2);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Mul { rd, rs1, rs2 } => {
            let v = cpu.read_gpr(rs1).wrapping_mul(cpu.read_gpr(rs2));
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Mulh { rd, rs1, rs2 } => {
            let a = cpu.read_gpr(rs1) as i64 as i128;
            let b = cpu.read_gpr(rs2) as i64 as i128;
            let v = ((a * b) >> 64) as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Mulhsu { rd, rs1, rs2 } => {
            let a = cpu.read_gpr(rs1) as i64 as i128;
            let b = cpu.read_gpr(rs2) as i128;
            let v = ((a * b) >> 64) as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Mulhu { rd, rs1, rs2 } => {
            let a = cpu.read_gpr(rs1) as u128;
            let b = cpu.read_gpr(rs2) as u128;
            let v = ((a * b) >> 64) as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Div { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1) as i64;
            let divisor = cpu.read_gpr(rs2) as i64;
            let v = if divisor == 0 {
                -1i64 as u64
            } else if dividend == i64::MIN && divisor == -1 {
                i64::MIN as u64
            } else {
                (dividend / divisor) as u64
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Divu { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1);
            let divisor = cpu.read_gpr(rs2);
            let v = dividend.checked_div(divisor).unwrap_or(u64::MAX);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Rem { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1) as i64;
            let divisor = cpu.read_gpr(rs2) as i64;
            let v = if divisor == 0 {
                dividend as u64
            } else if dividend == i64::MIN && divisor == -1 {
                0
            } else {
                (dividend % divisor) as u64
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Remu { rd, rs1, rs2 } => {
            let dividend = cpu.read_gpr(rs1);
            let divisor = cpu.read_gpr(rs2);
            let v = if divisor == 0 {
                dividend
            } else {
                dividend % divisor
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Sll { rd, rs1, rs2 } => {
            let sh = cpu.read_gpr(rs2) & 0x3f;
            let v = cpu.read_gpr(rs1) << sh;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Srl { rd, rs1, rs2 } => {
            let sh = cpu.read_gpr(rs2) & 0x3f;
            let v = cpu.read_gpr(rs1) >> sh;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Sra { rd, rs1, rs2 } => {
            let sh = cpu.read_gpr(rs2) & 0x3f;
            let v = (cpu.read_gpr(rs1) as i64 >> sh) as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Slt { rd, rs1, rs2 } => {
            let v = if (cpu.read_gpr(rs1) as i64) < (cpu.read_gpr(rs2) as i64) {
                1
            } else {
                0
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Sltu { rd, rs1, rs2 } => {
            let v = if cpu.read_gpr(rs1) < cpu.read_gpr(rs2) {
                1
            } else {
                0
            };
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Slli { rd, rs1, shamt } => {
            let v = cpu.read_gpr(rs1) << shamt;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Srli { rd, rs1, shamt } => {
            let v = cpu.read_gpr(rs1) >> shamt;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Srai { rd, rs1, shamt } => {
            let v = (cpu.read_gpr(rs1) as i64 >> shamt) as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Auipc { rd, imm } => {
            let v = cpu.pc.wrapping_add(imm as u64);
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Lui { rd, imm } => {
            cpu.write_gpr(rd, imm as u64);
            cpu.pc = npc;
        }
        crate::decode::Instr::Jal { rd, imm } => {
            cpu.write_gpr(rd, npc);
            cpu.pc = cpu.pc.wrapping_add(imm as u64);
        }
        crate::decode::Instr::Jalr { rd, rs1, imm } => {
            let target = (cpu.read_gpr(rs1).wrapping_add(imm as u64)) & !1u64;
            cpu.write_gpr(rd, npc);
            cpu.pc = target;
        }
        crate::decode::Instr::Beq { rs1, rs2, imm } => {
            if cpu.read_gpr(rs1) == cpu.read_gpr(rs2) {
                cpu.pc = cpu.pc.wrapping_add(imm as u64);
            } else {
                cpu.pc = npc;
            }
        }
        crate::decode::Instr::Bne { rs1, rs2, imm } => {
            if cpu.read_gpr(rs1) != cpu.read_gpr(rs2) {
                cpu.pc = cpu.pc.wrapping_add(imm as u64);
            } else {
                cpu.pc = npc;
            }
        }
        crate::decode::Instr::Blt { rs1, rs2, imm } => {
            if (cpu.read_gpr(rs1) as i64) < (cpu.read_gpr(rs2) as i64) {
                cpu.pc = cpu.pc.wrapping_add(imm as u64);
            } else {
                cpu.pc = npc;
            }
        }
        crate::decode::Instr::Bge { rs1, rs2, imm } => {
            if (cpu.read_gpr(rs1) as i64) >= (cpu.read_gpr(rs2) as i64) {
                cpu.pc = cpu.pc.wrapping_add(imm as u64);
            } else {
                cpu.pc = npc;
            }
        }
        crate::decode::Instr::Bltu { rs1, rs2, imm } => {
            if cpu.read_gpr(rs1) < cpu.read_gpr(rs2) {
                cpu.pc = cpu.pc.wrapping_add(imm as u64);
            } else {
                cpu.pc = npc;
            }
        }
        crate::decode::Instr::Bgeu { rs1, rs2, imm } => {
            if cpu.read_gpr(rs1) >= cpu.read_gpr(rs2) {
                cpu.pc = cpu.pc.wrapping_add(imm as u64);
            } else {
                cpu.pc = npc;
            }
        }
        crate::decode::Instr::Lb { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u8(a)? as i8 as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Lh { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u16(a)? as i16 as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Lw { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u32(a)? as i32 as i64 as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Ld { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u64(a)?;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Lbu { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u8(a)? as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Lhu { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u16(a)? as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Lwu { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u32(a)? as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Sb { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            cpu.clear_reservation_if_overlap(a, 1);
            mem.write_u8(a, cpu.read_gpr(rs2) as u8)?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Sh { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            cpu.clear_reservation_if_overlap(a, 2);
            mem.write_u16(a, cpu.read_gpr(rs2) as u16)?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Sw { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            cpu.clear_reservation_if_overlap(a, 4);
            let val = cpu.read_gpr(rs2) as u32;
            mem.write_u32(a, val)?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Sd { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            cpu.clear_reservation_if_overlap(a, 8);
            mem.write_u64(a, cpu.read_gpr(rs2))?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Flw { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u32(a)?;
            cpu.write_fpr_s(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Fld { rd, rs1, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let v = mem.read_u64(a)?;
            cpu.write_fpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::Fsw { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            cpu.clear_reservation_if_overlap(a, 4);
            let val = cpu.read_fpr_s(rs2);
            mem.write_u32(a, val)?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Fsd { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            cpu.clear_reservation_if_overlap(a, 8);
            let val = cpu.read_fpr(rs2);
            mem.write_u64(a, val)?;
            cpu.pc = npc;
        }
        // FP loads/stores already handled above; core FP arith/cvt/mv/cmp below
        crate::decode::Instr::FAddS {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = a + b;
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FAddD {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = a + b;
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FSubS {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = a - b;
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FSubD {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = a - b;
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FMulS {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = a * b;
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FMulD {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = a * b;
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FDivS {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = a / b;
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FDivD {
            rd,
            rs1,
            rs2,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = a / b;
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FSqrtS { rd, rs1, _rm: _ } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let res = a.sqrt();
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FSqrtD { rd, rs1, _rm: _ } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let res = a.sqrt();
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FMaddS {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let c = f32::from_bits(cpu.read_fpr_s(rs3));
            let res = a.mul_add(b, c);
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FMaddD {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let c = f64::from_bits(cpu.read_fpr(rs3));
            let res = a.mul_add(b, c);
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FMsubS {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let c = f32::from_bits(cpu.read_fpr_s(rs3));
            let res = a.mul_add(b, -c);
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FMsubD {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let c = f64::from_bits(cpu.read_fpr(rs3));
            let res = a.mul_add(b, -c);
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FNmaddS {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let c = f32::from_bits(cpu.read_fpr_s(rs3));
            let res = (-a).mul_add(b, -c);
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FNmaddD {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let c = f64::from_bits(cpu.read_fpr(rs3));
            let res = (-a).mul_add(b, -c);
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FNmsubS {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let c = f32::from_bits(cpu.read_fpr_s(rs3));
            let res = (-a).mul_add(b, c);
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FNmsubD {
            rd,
            rs1,
            rs2,
            rs3,
            _rm: _,
        } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let c = f64::from_bits(cpu.read_fpr(rs3));
            let res = (-a).mul_add(b, c);
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtWS { rd, rs1, _rm: _ } => {
            let f = f32::from_bits(cpu.read_fpr_s(rs1));
            let i = f as i32 as i64;
            cpu.write_gpr(rd, i as u64);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtWUS { rd, rs1, _rm: _ } => {
            let f = f32::from_bits(cpu.read_fpr_s(rs1));
            let u = if f <= 0.0 {
                0u32
            } else if f >= u32::MAX as f32 {
                u32::MAX
            } else {
                f as u32
            };
            cpu.write_gpr(rd, u as u64);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtLS { rd, rs1, _rm: _ } => {
            let f = f32::from_bits(cpu.read_fpr_s(rs1));
            let i = f as i64;
            cpu.write_gpr(rd, i as u64);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtLUS { rd, rs1, _rm: _ } => {
            let f = f32::from_bits(cpu.read_fpr_s(rs1));
            let u = if f <= 0.0 {
                0u64
            } else if f >= u64::MAX as f32 {
                u64::MAX
            } else {
                f as u64
            };
            cpu.write_gpr(rd, u);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtSW { rd, rs1, _rm: _ } => {
            let i = cpu.read_gpr(rs1) as i32 as f32;
            cpu.write_fpr_s(rd, i.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtSWU { rd, rs1, _rm: _ } => {
            let u = cpu.read_gpr(rs1) as u32 as f32;
            cpu.write_fpr_s(rd, u.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtSL { rd, rs1, _rm: _ } => {
            let i = cpu.read_gpr(rs1) as i64 as f32;
            cpu.write_fpr_s(rd, i.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtSLU { rd, rs1, _rm: _ } => {
            let u = cpu.read_gpr(rs1) as f32;
            cpu.write_fpr_s(rd, u.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtWD { rd, rs1, _rm: _ } => {
            let f = f64::from_bits(cpu.read_fpr(rs1));
            let i = f as i32 as i64;
            cpu.write_gpr(rd, i as u64);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtWUD { rd, rs1, _rm: _ } => {
            let f = f64::from_bits(cpu.read_fpr(rs1));
            let u = if f <= 0.0 {
                0u32
            } else if f >= u32::MAX as f64 {
                u32::MAX
            } else {
                f as u32
            };
            cpu.write_gpr(rd, u as u64);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtLD { rd, rs1, _rm: _ } => {
            let f = f64::from_bits(cpu.read_fpr(rs1));
            let i = f as i64;
            cpu.write_gpr(rd, i as u64);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtLUD { rd, rs1, _rm: _ } => {
            let f = f64::from_bits(cpu.read_fpr(rs1));
            let u = if f <= 0.0 {
                0u64
            } else if f >= u64::MAX as f64 {
                u64::MAX
            } else {
                f as u64
            };
            cpu.write_gpr(rd, u);
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtDW { rd, rs1, _rm: _ } => {
            let i = cpu.read_gpr(rs1) as i32 as f64;
            cpu.write_fpr(rd, i.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtDWU { rd, rs1, _rm: _ } => {
            let u = cpu.read_gpr(rs1) as u32 as f64;
            cpu.write_fpr(rd, u.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtDL { rd, rs1, _rm: _ } => {
            let i = cpu.read_gpr(rs1) as i64 as f64;
            cpu.write_fpr(rd, i.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtDLU { rd, rs1, _rm: _ } => {
            let u = cpu.read_gpr(rs1) as f64;
            cpu.write_fpr(rd, u.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtSD { rd, rs1, _rm: _ } => {
            let d = f64::from_bits(cpu.read_fpr(rs1));
            let s = d as f32;
            cpu.write_fpr_s(rd, s.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FcvtDS { rd, rs1, _rm: _ } => {
            let s = f32::from_bits(cpu.read_fpr_s(rs1));
            let d = s as f64;
            cpu.write_fpr(rd, d.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FmvXW { rd, rs1 } => {
            let v = cpu.read_fpr_s(rs1) as u64;
            cpu.write_gpr(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::FmvWX { rd, rs1 } => {
            let v = cpu.read_gpr(rs1) as u32;
            cpu.write_fpr_s(rd, v);
            cpu.pc = npc;
        }
        crate::decode::Instr::FmvXD { rd, rs1 } => {
            cpu.write_gpr(rd, cpu.read_fpr(rs1));
            cpu.pc = npc;
        }
        crate::decode::Instr::FmvDX { rd, rs1 } => {
            cpu.write_fpr(rd, cpu.read_gpr(rs1));
            cpu.pc = npc;
        }
        crate::decode::Instr::FsgnjS { rd, rs1, rs2 } => {
            let mut a = cpu.read_fpr_s(rs1);
            let b = cpu.read_fpr_s(rs2);
            a = (a & 0x7fffffff) | (b & 0x80000000);
            cpu.write_fpr_s(rd, a);
            cpu.pc = npc;
        }
        crate::decode::Instr::FsgnjD { rd, rs1, rs2 } => {
            let mut a = cpu.read_fpr(rs1);
            let b = cpu.read_fpr(rs2);
            a = (a & 0x7fff_ffff_ffff_ffff) | (b & 0x8000_0000_0000_0000);
            cpu.write_fpr(rd, a);
            cpu.pc = npc;
        }
        crate::decode::Instr::FsgnjnS { rd, rs1, rs2 } => {
            let mut a = cpu.read_fpr_s(rs1);
            let b = cpu.read_fpr_s(rs2);
            a = (a & 0x7fffffff) | ((!b) & 0x80000000);
            cpu.write_fpr_s(rd, a);
            cpu.pc = npc;
        }
        crate::decode::Instr::FsgnjnD { rd, rs1, rs2 } => {
            let mut a = cpu.read_fpr(rs1);
            let b = cpu.read_fpr(rs2);
            a = (a & 0x7fff_ffff_ffff_ffff) | ((!b) & 0x8000_0000_0000_0000);
            cpu.write_fpr(rd, a);
            cpu.pc = npc;
        }
        crate::decode::Instr::FsgnjxS { rd, rs1, rs2 } => {
            let mut a = cpu.read_fpr_s(rs1);
            let b = cpu.read_fpr_s(rs2);
            a = (a & 0x7fffffff) | ((a ^ b) & 0x80000000);
            cpu.write_fpr_s(rd, a);
            cpu.pc = npc;
        }
        crate::decode::Instr::FsgnjxD { rd, rs1, rs2 } => {
            let mut a = cpu.read_fpr(rs1);
            let b = cpu.read_fpr(rs2);
            a = (a & 0x7fff_ffff_ffff_ffff) | ((a ^ b) & 0x8000_0000_0000_0000);
            cpu.write_fpr(rd, a);
            cpu.pc = npc;
        }
        crate::decode::Instr::FminS { rd, rs1, rs2 } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = if a.is_nan() {
                b
            } else if b.is_nan() || a < b {
                a
            } else {
                b
            };
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FminD { rd, rs1, rs2 } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = if a.is_nan() {
                b
            } else if b.is_nan() || a < b {
                a
            } else {
                b
            };
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FmaxS { rd, rs1, rs2 } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = if a.is_nan() {
                b
            } else if b.is_nan() || a > b {
                a
            } else {
                b
            };
            cpu.write_fpr_s(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FmaxD { rd, rs1, rs2 } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = if a.is_nan() {
                b
            } else if b.is_nan() || a > b {
                a
            } else {
                b
            };
            cpu.write_fpr(rd, res.to_bits());
            cpu.pc = npc;
        }
        crate::decode::Instr::FeqS { rd, rs1, rs2 } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = if a == b { 1 } else { 0 };
            cpu.write_gpr(rd, res);
            cpu.pc = npc;
        }
        crate::decode::Instr::FeqD { rd, rs1, rs2 } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = if a == b { 1 } else { 0 };
            cpu.write_gpr(rd, res);
            cpu.pc = npc;
        }
        crate::decode::Instr::FltS { rd, rs1, rs2 } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = if a < b { 1 } else { 0 };
            cpu.write_gpr(rd, res);
            cpu.pc = npc;
        }
        crate::decode::Instr::FltD { rd, rs1, rs2 } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = if a < b { 1 } else { 0 };
            cpu.write_gpr(rd, res);
            cpu.pc = npc;
        }
        crate::decode::Instr::FleS { rd, rs1, rs2 } => {
            let a = f32::from_bits(cpu.read_fpr_s(rs1));
            let b = f32::from_bits(cpu.read_fpr_s(rs2));
            let res = if a <= b { 1 } else { 0 };
            cpu.write_gpr(rd, res);
            cpu.pc = npc;
        }
        crate::decode::Instr::FleD { rd, rs1, rs2 } => {
            let a = f64::from_bits(cpu.read_fpr(rs1));
            let b = f64::from_bits(cpu.read_fpr(rs2));
            let res = if a <= b { 1 } else { 0 };
            cpu.write_gpr(rd, res);
            cpu.pc = npc;
        }
        crate::decode::Instr::FclassS { rd, rs1 } => {
            let bits = cpu.read_fpr_s(rs1);
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
            cpu.write_gpr(rd, cls);
            cpu.pc = npc;
        }
        crate::decode::Instr::FclassD { rd, rs1 } => {
            let bits = cpu.read_fpr(rs1);
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
            cpu.write_gpr(rd, cls);
            cpu.pc = npc;
        }
        crate::decode::Instr::LrW {
            rd,
            rs1,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            let v = mem.read_u32(a)? as i32 as u64;
            cpu.write_gpr(rd, v);
            cpu.set_reservation(a, 4);
            cpu.pc = npc;
        }
        crate::decode::Instr::LrD {
            rd,
            rs1,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            let v = mem.read_u64(a)?;
            cpu.write_gpr(rd, v);
            cpu.set_reservation(a, 8);
            cpu.pc = npc;
        }
        crate::decode::Instr::ScW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            let v = cpu.read_gpr(rs2) as u32;
            let succ = if cpu.check_and_clear_reservation(a, 4) {
                mem.write_u32(a, v)?;
                0
            } else {
                1
            };
            cpu.write_gpr(rd, succ);
            cpu.pc = npc;
        }
        crate::decode::Instr::ScD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            let v = cpu.read_gpr(rs2);
            let succ = if cpu.check_and_clear_reservation(a, 8) {
                mem.write_u64(a, v)?;
                0
            } else {
                1
            };
            cpu.write_gpr(rd, succ);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoSwapW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as i32 as u64;
            let nv = cpu.read_gpr(rs2) as u32;
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoSwapD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let nv = cpu.read_gpr(rs2);
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoAddW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as i32 as u64;
            let addv = cpu.read_gpr(rs2) as u32;
            let nv = (old as u32).wrapping_add(addv);
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoAddD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let nv = old.wrapping_add(cpu.read_gpr(rs2));
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoXorW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as i32 as u64;
            let nv = (old as u32) ^ (cpu.read_gpr(rs2) as u32);
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoXorD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let nv = old ^ cpu.read_gpr(rs2);
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoAndW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as i32 as u64;
            let nv = (old as u32) & (cpu.read_gpr(rs2) as u32);
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoAndD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let nv = old & cpu.read_gpr(rs2);
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoOrW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as i32 as u64;
            let nv = (old as u32) | (cpu.read_gpr(rs2) as u32);
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoOrD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let nv = old | cpu.read_gpr(rs2);
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMinW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as i32 as u64;
            let v = cpu.read_gpr(rs2) as i32 as u64;
            let nv = if (old as i32) < (v as i32) {
                old as u32
            } else {
                v as u32
            };
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMinD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let v = cpu.read_gpr(rs2);
            let nv = if (old as i64) < (v as i64) { old } else { v };
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMaxW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as i32 as u64;
            let v = cpu.read_gpr(rs2) as i32 as u64;
            let nv = if (old as i32) > (v as i32) {
                old as u32
            } else {
                v as u32
            };
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMaxD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let v = cpu.read_gpr(rs2);
            let nv = if (old as i64) > (v as i64) { old } else { v };
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMinuW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as u64;
            let v = cpu.read_gpr(rs2) as u32 as u64;
            let nv = if (old as u32) < (v as u32) {
                old as u32
            } else {
                v as u32
            };
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMinuD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let v = cpu.read_gpr(rs2);
            let nv = if old < v { old } else { v };
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMaxuW {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 4);
            let old = mem.read_u32(a)? as u64;
            let v = cpu.read_gpr(rs2) as u32 as u64;
            let nv = if (old as u32) > (v as u32) {
                old as u32
            } else {
                v as u32
            };
            mem.write_u32(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::AmoMaxuD {
            rd,
            rs1,
            rs2,
            _aq: _,
            _rl: _,
        } => {
            let a = cpu.read_gpr(rs1);
            cpu.clear_reservation_if_overlap(a, 8);
            let old = mem.read_u64(a)?;
            let v = cpu.read_gpr(rs2);
            let nv = if old > v { old } else { v };
            mem.write_u64(a, nv)?;
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::Ecall => {
            if let Some(code) = crate::syscall::handle_ecall(cpu, mem)? {
                std::process::exit(code);
            }
            cpu.pc = npc;
        }
        crate::decode::Instr::Mret => {
            let mpp = (cpu.mstatus >> 11) & 3;
            cpu.priv_mode = mpp;
            cpu.mstatus &= !(3u64 << 11);
            cpu.pc = cpu.mepc;
        }
        crate::decode::Instr::Fence => {
            cpu.pc = npc;
        }
        crate::decode::Instr::CsrRW { rd, rs1, csr } => {
            let old = cpu.read_csr(csr);
            let val = cpu.read_gpr(rs1);
            cpu.write_csr(csr, val);
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::CsrRS { rd, rs1, csr } => {
            let old = cpu.read_csr(csr);
            let val = cpu.read_gpr(rs1);
            cpu.write_csr(csr, old | val);
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::CsrRC { rd, rs1, csr } => {
            let old = cpu.read_csr(csr);
            let val = cpu.read_gpr(rs1);
            cpu.write_csr(csr, old & !val);
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::CsrRWI { rd, zimm, csr } => {
            let old = cpu.read_csr(csr);
            let val = zimm as u64;
            cpu.write_csr(csr, val);
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::CsrRSI { rd, zimm, csr } => {
            let old = cpu.read_csr(csr);
            let val = zimm as u64;
            cpu.write_csr(csr, old | val);
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::CsrRCI { rd, zimm, csr } => {
            let old = cpu.read_csr(csr);
            let val = zimm as u64;
            cpu.write_csr(csr, old & !val);
            cpu.write_gpr(rd, old);
            cpu.pc = npc;
        }
        crate::decode::Instr::Unknown(raw) => {
            anyhow::bail!("unknown instruction {:#x} at pc {:#x}", raw, cpu.pc);
        }
    }
    Ok(())
}
