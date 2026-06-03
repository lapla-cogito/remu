pub enum Instr {
    Addi { rd: u8, rs1: u8, imm: i64 },
    Auipc { rd: u8, imm: i64 },
    Ecall,
    Unknown(u32),
}

fn sign_extend(val: u64, bits: u32) -> u64 {
    let shift = 64 - bits;
    ((val << shift) as i64 >> shift) as u64
}

pub fn fetch_decode(mem: &crate::memory::GuestMemory, pc: u64) -> anyhow::Result<(u32, Instr)> {
    let low = mem.read_u16(pc)? as u32;
    if (low & 3) != 3 {
        let instr = decode_compressed(low as u16);
        Ok((2, instr))
    } else {
        let high = mem.read_u16(pc + 2)? as u32;
        let raw = low | (high << 16);
        let instr = decode(raw);
        Ok((4, instr))
    }
}

pub fn decode(raw: u32) -> Instr {
    let opcode = raw & 0x7f;
    let rd = ((raw >> 7) & 0x1f) as u8;
    let rs1 = ((raw >> 15) & 0x1f) as u8;
    let funct3 = (raw >> 12) & 7;
    match opcode {
        0x13 => {
            if funct3 == 0 {
                let imm12 = (raw >> 20) & 0xfff;
                let imm = sign_extend(imm12 as u64, 12) as i64;
                Instr::Addi { rd, rs1, imm }
            } else {
                Instr::Unknown(raw)
            }
        }
        0x17 => {
            let imm20 = raw & 0xfffff000;
            let imm = sign_extend(imm20 as u64, 32) as i64;
            Instr::Auipc { rd, imm }
        }
        0x73 => {
            if raw == 0x00000073 {
                Instr::Ecall
            } else {
                Instr::Unknown(raw)
            }
        }
        _ => Instr::Unknown(raw),
    }
}

pub fn decode_compressed(raw: u16) -> Instr {
    let q = raw & 3;
    let f3 = (raw >> 13) & 7;
    match (q, f3) {
        (1, 0) => {
            let rd = ((raw >> 7) & 0x1f) as u8;
            let imm5 = ((raw >> 12) & 1) as u64;
            let imm40 = ((raw >> 2) & 0x1f) as u64;
            let imm = sign_extend((imm5 << 5) | imm40, 6) as i64;
            if rd != 0 {
                Instr::Addi { rd, rs1: rd, imm }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (2, 0) => {
            let rd = ((raw >> 7) & 0x1f) as u8;
            let imm5 = ((raw >> 12) & 1) as u64;
            let imm40 = ((raw >> 2) & 0x1f) as u64;
            let imm = sign_extend((imm5 << 5) | imm40, 6) as i64;
            if rd != 0 {
                Instr::Addi { rd, rs1: 0, imm }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        _ => Instr::Unknown(raw as u32),
    }
}