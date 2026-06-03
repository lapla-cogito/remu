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
            cpu.write_gpr(rd, npc);
            let target = (cpu.read_gpr(rs1).wrapping_add(imm as u64)) & !1u64;
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
            mem.write_u8(a, cpu.read_gpr(rs2) as u8)?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Sh { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            mem.write_u16(a, cpu.read_gpr(rs2) as u16)?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Sw { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            let val = cpu.read_gpr(rs2) as u32;
            mem.write_u32(a, val)?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Sd { rs1, rs2, imm } => {
            let a = cpu.read_gpr(rs1).wrapping_add(imm as u64);
            mem.write_u64(a, cpu.read_gpr(rs2))?;
            cpu.pc = npc;
        }
        crate::decode::Instr::Ecall => {
            if let Some(code) = crate::syscall::handle_ecall(cpu, mem)? {
                std::process::exit(code);
            }
            cpu.pc = npc;
        }
        crate::decode::Instr::Unknown(raw) => {
            anyhow::bail!("unknown instruction {:#x} at pc {:#x}", raw, cpu.pc);
        }
    }
    Ok(())
}
