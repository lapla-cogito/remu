pub enum Instr {
    Addi {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Addiw {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Slliw {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Srliw {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Sraiw {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Add {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sub {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    And {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Or {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Xor {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Andi {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Ori {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Xori {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Slti {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Sltiu {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Sll {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Srl {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sra {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Slt {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sltu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mul {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mulh {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mulhsu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mulhu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Div {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Divu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Rem {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Remu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mulw {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Divw {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Divuw {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Remw {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Remuw {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Slli {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Srli {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Srai {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Auipc {
        rd: u8,
        imm: i64,
    },
    Lui {
        rd: u8,
        imm: i64,
    },
    Jal {
        rd: u8,
        imm: i64,
    },
    Jalr {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Beq {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Bne {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Blt {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Bge {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Bltu {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Bgeu {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Lb {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Lh {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Lw {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Ld {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Lbu {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Lhu {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Lwu {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Sb {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Sh {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Sw {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Sd {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Flw {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Fld {
        rd: u8,
        rs1: u8,
        imm: i64,
    },
    Fsw {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    Fsd {
        rs1: u8,
        rs2: u8,
        imm: i64,
    },
    FAddS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FAddD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FSubS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FSubD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FMulS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FMulD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FDivS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FDivD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        _rm: u8,
    },
    FSqrtS {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FSqrtD {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FMaddS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FMaddD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FMsubS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FMsubD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FNmaddS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FNmaddD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FNmsubS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FNmsubD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        _rm: u8,
    },
    FcvtWS {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtWUS {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtSW {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtSWU {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtWD {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtWUD {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtSD {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtDS {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtDW {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtDWU {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtLS {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtLUS {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtSL {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtSLU {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtLD {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtLUD {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtDL {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FcvtDLU {
        rd: u8,
        rs1: u8,
        _rm: u8,
    },
    FmvXW {
        rd: u8,
        rs1: u8,
    },
    FmvWX {
        rd: u8,
        rs1: u8,
    },
    FmvXD {
        rd: u8,
        rs1: u8,
    },
    FmvDX {
        rd: u8,
        rs1: u8,
    },
    FsgnjS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjnS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjnD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjxS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjxD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FminS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FminD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FmaxS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FmaxD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FeqS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FeqD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FltS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FltD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FleS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FleD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FclassS {
        rd: u8,
        rs1: u8,
    },
    FclassD {
        rd: u8,
        rs1: u8,
    },
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
    let rs2 = ((raw >> 20) & 0x1f) as u8;
    let funct3 = (raw >> 12) & 7;
    let funct7 = (raw >> 25) & 0x7f;
    match opcode {
        0x13 => {
            let imm12 = (raw >> 20) & 0xfff;
            let imm = sign_extend(imm12 as u64, 12) as i64;
            match funct3 {
                0 => Instr::Addi { rd, rs1, imm },
                1 => {
                    let shamt = (raw >> 20) & 0x3f;
                    Instr::Slli { rd, rs1, shamt }
                }
                2 => Instr::Slti { rd, rs1, imm },
                3 => Instr::Sltiu { rd, rs1, imm },
                4 => Instr::Xori { rd, rs1, imm },
                5 => {
                    let shamt = (raw >> 20) & 0x3f;
                    if funct7 == 0 {
                        Instr::Srli { rd, rs1, shamt }
                    } else {
                        Instr::Srai { rd, rs1, shamt }
                    }
                }
                6 => Instr::Ori { rd, rs1, imm },
                7 => Instr::Andi { rd, rs1, imm },
                _ => Instr::Unknown(raw),
            }
        }
        0x1b => {
            let imm12 = (raw >> 20) & 0xfff;
            let imm = sign_extend(imm12 as u64, 12) as i64;
            match funct3 {
                0 => Instr::Addiw { rd, rs1, imm },
                1 => {
                    let shamt = (raw >> 20) & 0x3f;
                    Instr::Slliw { rd, rs1, shamt }
                }
                5 => {
                    let shamt = (raw >> 20) & 0x3f;
                    if funct7 == 0 {
                        Instr::Srliw { rd, rs1, shamt }
                    } else {
                        Instr::Sraiw { rd, rs1, shamt }
                    }
                }
                _ => Instr::Unknown(raw),
            }
        }
        0x17 => {
            let imm20 = raw & 0xfffff000;
            let imm = sign_extend(imm20 as u64, 32) as i64;
            Instr::Auipc { rd, imm }
        }
        0x33 => match (funct3, funct7) {
            (0, 0) => Instr::Add { rd, rs1, rs2 },
            (0, 0x20) => Instr::Sub { rd, rs1, rs2 },
            (1, 0) => Instr::Sll { rd, rs1, rs2 },
            (2, 0) => Instr::Slt { rd, rs1, rs2 },
            (3, 0) => Instr::Sltu { rd, rs1, rs2 },
            (4, 0) => Instr::Xor { rd, rs1, rs2 },
            (5, 0) => Instr::Srl { rd, rs1, rs2 },
            (5, 0x20) => Instr::Sra { rd, rs1, rs2 },
            (6, 0) => Instr::Or { rd, rs1, rs2 },
            (7, 0) => Instr::And { rd, rs1, rs2 },
            (0, 1) => Instr::Mul { rd, rs1, rs2 },
            (1, 1) => Instr::Mulh { rd, rs1, rs2 },
            (2, 1) => Instr::Mulhsu { rd, rs1, rs2 },
            (3, 1) => Instr::Mulhu { rd, rs1, rs2 },
            (4, 1) => Instr::Div { rd, rs1, rs2 },
            (5, 1) => Instr::Divu { rd, rs1, rs2 },
            (6, 1) => Instr::Rem { rd, rs1, rs2 },
            (7, 1) => Instr::Remu { rd, rs1, rs2 },
            _ => Instr::Unknown(raw),
        },
        0x3b => match (funct3, funct7) {
            (0, 1) => Instr::Mulw { rd, rs1, rs2 },
            (4, 1) => Instr::Divw { rd, rs1, rs2 },
            (5, 1) => Instr::Divuw { rd, rs1, rs2 },
            (6, 1) => Instr::Remw { rd, rs1, rs2 },
            (7, 1) => Instr::Remuw { rd, rs1, rs2 },
            _ => Instr::Unknown(raw),
        },
        0x37 => {
            let imm20 = raw & 0xfffff000;
            let imm = sign_extend(imm20 as u64, 32) as i64;
            Instr::Lui { rd, imm }
        }
        0x63 => {
            let imm12 = ((raw >> 31) & 1) << 12
                | ((raw >> 7) & 1) << 11
                | ((raw >> 25) & 0x3f) << 5
                | ((raw >> 8) & 0xf) << 1;
            let imm = sign_extend(imm12 as u64, 13) as i64;
            match funct3 {
                0 => Instr::Beq { rs1, rs2, imm },
                1 => Instr::Bne { rs1, rs2, imm },
                4 => Instr::Blt { rs1, rs2, imm },
                5 => Instr::Bge { rs1, rs2, imm },
                6 => Instr::Bltu { rs1, rs2, imm },
                7 => Instr::Bgeu { rs1, rs2, imm },
                _ => Instr::Unknown(raw),
            }
        }
        0x67 => {
            let imm12 = (raw >> 20) & 0xfff;
            let imm = sign_extend(imm12 as u64, 12) as i64;
            Instr::Jalr { rd, rs1, imm }
        }
        0x6f => {
            let imm20 = ((raw >> 31) & 1) << 20
                | ((raw >> 21) & 0x3ff) << 1
                | ((raw >> 20) & 1) << 11
                | ((raw >> 12) & 0xff) << 12;
            let imm = sign_extend(imm20 as u64, 21) as i64;
            Instr::Jal { rd, imm }
        }
        0x03 => {
            let imm12 = (raw >> 20) & 0xfff;
            let imm = sign_extend(imm12 as u64, 12) as i64;
            match funct3 {
                0 => Instr::Lb { rd, rs1, imm },
                1 => Instr::Lh { rd, rs1, imm },
                2 => Instr::Lw { rd, rs1, imm },
                3 => Instr::Ld { rd, rs1, imm },
                4 => Instr::Lbu { rd, rs1, imm },
                5 => Instr::Lhu { rd, rs1, imm },
                6 => Instr::Lwu { rd, rs1, imm },
                _ => Instr::Unknown(raw),
            }
        }
        0x23 => {
            let imm12 = ((raw >> 25) & 0x7f) << 5 | ((raw >> 7) & 0x1f);
            let imm = sign_extend(imm12 as u64, 12) as i64;
            match funct3 {
                0 => Instr::Sb { rs1, rs2, imm },
                1 => Instr::Sh { rs1, rs2, imm },
                2 => Instr::Sw { rs1, rs2, imm },
                3 => Instr::Sd { rs1, rs2, imm },
                _ => Instr::Unknown(raw),
            }
        }
        0x07 => {
            let imm12 = (raw >> 20) & 0xfff;
            let imm = sign_extend(imm12 as u64, 12) as i64;
            match funct3 {
                2 => Instr::Flw { rd, rs1, imm },
                3 => Instr::Fld { rd, rs1, imm },
                _ => Instr::Unknown(raw),
            }
        }
        0x27 => {
            let imm12 = ((raw >> 25) & 0x7f) << 5 | ((raw >> 7) & 0x1f);
            let imm = sign_extend(imm12 as u64, 12) as i64;
            match funct3 {
                2 => Instr::Fsw { rs1, rs2, imm },
                3 => Instr::Fsd { rs1, rs2, imm },
                _ => Instr::Unknown(raw),
            }
        }
        0x43 | 0x47 | 0x4b | 0x4f => {
            let rs3 = ((raw >> 27) & 0x1f) as u8;
            let rm = ((raw >> 12) & 7) as u8;
            let is_d = ((raw >> 25) & 1) == 1;
            match opcode {
                0x43 => {
                    if is_d {
                        Instr::FMaddD {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    } else {
                        Instr::FMaddS {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    }
                }
                0x47 => {
                    if is_d {
                        Instr::FMsubD {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    } else {
                        Instr::FMsubS {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    }
                }
                0x4b => {
                    if is_d {
                        Instr::FNmsubD {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    } else {
                        Instr::FNmsubS {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    }
                }
                0x4f => {
                    if is_d {
                        Instr::FNmaddD {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    } else {
                        Instr::FNmaddS {
                            rd,
                            rs1,
                            rs2,
                            rs3,
                            _rm: rm,
                        }
                    }
                }
                _ => Instr::Unknown(raw),
            }
        }
        0x53 => {
            let rm = funct3 as u8;
            match funct7 {
                0 => Instr::FAddS {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                1 => Instr::FAddD {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                4 => Instr::FSubS {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                5 => Instr::FSubD {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                8 => Instr::FMulS {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                9 => Instr::FMulD {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                12 => Instr::FDivS {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                13 => Instr::FDivD {
                    rd,
                    rs1,
                    rs2,
                    _rm: rm,
                },
                44 => Instr::FSqrtS { rd, rs1, _rm: rm },
                45 => Instr::FSqrtD { rd, rs1, _rm: rm },
                16 => match rm {
                    0 => Instr::FsgnjS { rd, rs1, rs2 },
                    1 => Instr::FsgnjnS { rd, rs1, rs2 },
                    2 => Instr::FsgnjxS { rd, rs1, rs2 },
                    _ => Instr::Unknown(raw),
                },
                17 => match rm {
                    0 => Instr::FsgnjD { rd, rs1, rs2 },
                    1 => Instr::FsgnjnD { rd, rs1, rs2 },
                    2 => Instr::FsgnjxD { rd, rs1, rs2 },
                    _ => Instr::Unknown(raw),
                },
                20 => match rm {
                    0 => Instr::FminS { rd, rs1, rs2 },
                    1 => Instr::FmaxS { rd, rs1, rs2 },
                    _ => Instr::Unknown(raw),
                },
                21 => match rm {
                    0 => Instr::FminD { rd, rs1, rs2 },
                    1 => Instr::FmaxD { rd, rs1, rs2 },
                    _ => Instr::Unknown(raw),
                },
                80 => match rm {
                    0 => Instr::FleS { rd, rs1, rs2 },
                    1 => Instr::FltS { rd, rs1, rs2 },
                    2 => Instr::FeqS { rd, rs1, rs2 },
                    _ => Instr::Unknown(raw),
                },
                81 => match rm {
                    0 => Instr::FleD { rd, rs1, rs2 },
                    1 => Instr::FltD { rd, rs1, rs2 },
                    2 => Instr::FeqD { rd, rs1, rs2 },
                    _ => Instr::Unknown(raw),
                },
                32 => {
                    if rs2 == 1 {
                        Instr::FcvtSD { rd, rs1, _rm: rm }
                    } else {
                        Instr::FcvtDS { rd, rs1, _rm: rm }
                    }
                }
                33 => {
                    if rs2 == 0 {
                        Instr::FcvtDS { rd, rs1, _rm: rm }
                    } else {
                        Instr::FcvtSD { rd, rs1, _rm: rm }
                    }
                }
                96 => match rs2 {
                    0 => Instr::FcvtWS { rd, rs1, _rm: rm },
                    1 => Instr::FcvtWUS { rd, rs1, _rm: rm },
                    2 => Instr::FcvtLS { rd, rs1, _rm: rm },
                    3 => Instr::FcvtLUS { rd, rs1, _rm: rm },
                    _ => Instr::Unknown(raw),
                },
                97 => match rs2 {
                    0 => Instr::FcvtWD { rd, rs1, _rm: rm },
                    1 => Instr::FcvtWUD { rd, rs1, _rm: rm },
                    2 => Instr::FcvtLD { rd, rs1, _rm: rm },
                    3 => Instr::FcvtLUD { rd, rs1, _rm: rm },
                    _ => Instr::Unknown(raw),
                },
                104 => match rs2 {
                    0 => Instr::FcvtSW { rd, rs1, _rm: rm },
                    1 => Instr::FcvtSWU { rd, rs1, _rm: rm },
                    2 => Instr::FcvtSL { rd, rs1, _rm: rm },
                    3 => Instr::FcvtSLU { rd, rs1, _rm: rm },
                    _ => Instr::Unknown(raw),
                },
                105 => match rs2 {
                    0 => Instr::FcvtDW { rd, rs1, _rm: rm },
                    1 => Instr::FcvtDWU { rd, rs1, _rm: rm },
                    2 => Instr::FcvtDL { rd, rs1, _rm: rm },
                    3 => Instr::FcvtDLU { rd, rs1, _rm: rm },
                    _ => Instr::Unknown(raw),
                },
                112 => match rm {
                    0 => Instr::FmvXD { rd, rs1 },
                    1 => Instr::FclassD { rd, rs1 },
                    _ => Instr::Unknown(raw),
                },
                113 => match rm {
                    0 => Instr::FmvXW { rd, rs1 },
                    1 => Instr::FclassS { rd, rs1 },
                    _ => Instr::Unknown(raw),
                },
                120 => Instr::FmvDX { rd, rs1 },
                121 => Instr::FmvWX { rd, rs1 },
                _ => Instr::Unknown(raw),
            }
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
        (1, 2) => {
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
        (1, 3) => {
            let rd = ((raw >> 7) & 0x1f) as u8;
            if rd == 2 {
                // c.addi16sp
                let mut imm = 0u64;
                imm |= (((raw >> 12) & 1) as u64) << 9;
                imm |= (((raw >> 3) & 3) as u64) << 7;
                imm |= (((raw >> 5) & 1) as u64) << 6;
                imm |= (((raw >> 2) & 1) as u64) << 5;
                imm |= (((raw >> 4) & 1) as u64) << 4;
                let imm = sign_extend(imm, 10) as i64;
                Instr::Addi { rd: 2, rs1: 2, imm }
            } else if rd != 0 {
                let imm5 = ((raw >> 12) & 1) as u64;
                let imm40 = ((raw >> 2) & 0x1f) as u64;
                let imm = sign_extend((imm5 << 5) | imm40, 6) as i64;
                Instr::Lui { rd, imm: imm << 12 }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (1, 4) => {
            let bit12 = (raw >> 12) & 1;
            let funct2 = (raw >> 10) & 3;
            let rdp = 8 + ((raw >> 7) & 7) as u8;
            if funct2 == 0 {
                let shamt = ((raw >> 2) & 0x1f) as u32;
                Instr::Srli {
                    rd: rdp,
                    rs1: rdp,
                    shamt,
                }
            } else if funct2 == 1 {
                let shamt = ((raw >> 2) & 0x1f) as u32;
                Instr::Srai {
                    rd: rdp,
                    rs1: rdp,
                    shamt,
                }
            } else if funct2 == 2 {
                let imm5 = bit12 as u64;
                let imm40 = ((raw >> 2) & 0x1f) as u64;
                let imm = sign_extend((imm5 << 5) | imm40, 6) as i64;
                Instr::Andi {
                    rd: rdp,
                    rs1: rdp,
                    imm,
                }
            } else if funct2 == 3 && bit12 == 0 {
                let rs2p = 8 + ((raw >> 2) & 7) as u8;
                let f2b = (raw >> 5) & 3;
                match f2b {
                    0 => Instr::Sub {
                        rd: rdp,
                        rs1: rdp,
                        rs2: rs2p,
                    },
                    1 => Instr::Xor {
                        rd: rdp,
                        rs1: rdp,
                        rs2: rs2p,
                    },
                    2 => Instr::Or {
                        rd: rdp,
                        rs1: rdp,
                        rs2: rs2p,
                    },
                    3 => Instr::And {
                        rd: rdp,
                        rs1: rdp,
                        rs2: rs2p,
                    },
                    _ => Instr::Unknown(raw as u32),
                }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (1, 1) => {
            let rd = ((raw >> 7) & 0x1f) as u8;
            if rd != 0 {
                let imm5 = ((raw >> 12) & 1) as u64;
                let imm40 = ((raw >> 2) & 0x1f) as u64;
                let imm = sign_extend((imm5 << 5) | imm40, 6) as i64;
                Instr::Addiw { rd, rs1: rd, imm }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (1, 5) => {
            // c.j
            let mut imm = 0u64;
            imm |= (((raw >> 12) & 1) as u64) << 11;
            imm |= (((raw >> 8) & 1) as u64) << 10;
            imm |= (((raw >> 9) & 3) as u64) << 8;
            imm |= (((raw >> 6) & 1) as u64) << 7;
            imm |= (((raw >> 7) & 1) as u64) << 6;
            imm |= (((raw >> 2) & 1) as u64) << 5;
            imm |= (((raw >> 11) & 1) as u64) << 4;
            imm |= (((raw >> 3) & 7) as u64) << 1;
            let imm = sign_extend(imm, 12) as i64;
            Instr::Jal { rd: 0, imm }
        }
        (1, 6) => {
            // c.beqz
            let rs1 = 8 + ((raw >> 7) & 7) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 12) & 1) as u64) << 8;
            imm |= (((raw >> 5) & 3) as u64) << 6;
            imm |= (((raw >> 2) & 1) as u64) << 5;
            imm |= (((raw >> 10) & 3) as u64) << 3;
            imm |= (((raw >> 3) & 3) as u64) << 1;
            let imm = sign_extend(imm, 9) as i64;
            Instr::Beq { rs1, rs2: 0, imm }
        }
        (1, 7) => {
            // c.bnez
            let rs1 = 8 + ((raw >> 7) & 7) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 12) & 1) as u64) << 8;
            imm |= (((raw >> 5) & 3) as u64) << 6;
            imm |= (((raw >> 2) & 1) as u64) << 5;
            imm |= (((raw >> 10) & 3) as u64) << 3;
            imm |= (((raw >> 3) & 3) as u64) << 1;
            let imm = sign_extend(imm, 9) as i64;
            Instr::Bne { rs1, rs2: 0, imm }
        }
        (0, 0) => {
            // c.addi4spn
            let rdp = 8 + ((raw >> 2) & 7) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 5) & 1) as u64) << 3;
            imm |= (((raw >> 6) & 1) as u64) << 2;
            imm |= (((raw >> 10) & 7) as u64) << 6;
            imm |= (((raw >> 7) & 3) as u64) << 4;
            let imm = sign_extend(imm, 10) as i64; // nzuimm
            if rdp != 0 && imm != 0 {
                Instr::Addi {
                    rd: rdp,
                    rs1: 2,
                    imm,
                }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (0, 2) => {
            // c.lw
            let rdp = 8 + ((raw >> 2) & 7) as u8;
            let rs1p = 8 + ((raw >> 7) & 7) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 6) & 1) as u64) << 2;
            imm |= (((raw >> 10) & 7) as u64) << 3;
            imm |= (((raw >> 5) & 1) as u64) << 6;
            Instr::Lw {
                rd: rdp,
                rs1: rs1p,
                imm: imm as i64,
            }
        }
        (0, 3) => {
            // c.ld
            let rdp = 8 + ((raw >> 2) & 7) as u8;
            let rs1p = 8 + ((raw >> 7) & 7) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 6) & 1) as u64) << 3;
            imm |= (((raw >> 10) & 7) as u64) << 4;
            imm |= (((raw >> 5) & 1) as u64) << 7;
            Instr::Ld {
                rd: rdp,
                rs1: rs1p,
                imm: imm as i64,
            }
        }
        (0, 6) => {
            // c.sw
            let rs2p = 8 + ((raw >> 2) & 7) as u8;
            let rs1p = 8 + ((raw >> 7) & 7) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 6) & 1) as u64) << 2;
            imm |= (((raw >> 10) & 7) as u64) << 3;
            imm |= (((raw >> 5) & 1) as u64) << 6;
            Instr::Sw {
                rs1: rs1p,
                rs2: rs2p,
                imm: imm as i64,
            }
        }
        (0, 7) => {
            // c.sd
            let rs2p = 8 + ((raw >> 2) & 7) as u8;
            let rs1p = 8 + ((raw >> 7) & 7) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 6) & 1) as u64) << 3;
            imm |= (((raw >> 10) & 7) as u64) << 4;
            imm |= (((raw >> 5) & 1) as u64) << 7;
            Instr::Sd {
                rs1: rs1p,
                rs2: rs2p,
                imm: imm as i64,
            }
        }
        (2, 0) => {
            // c.slli
            let rd = ((raw >> 7) & 0x1f) as u8;
            let shamt = ((raw >> 2) & 0x3f) as u32;
            if rd != 0 {
                Instr::Slli { rd, rs1: rd, shamt }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (2, 2) => {
            // c.lwsp
            let rd = ((raw >> 7) & 0x1f) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 2) & 3) as u64) << 6;
            imm |= (((raw >> 4) & 3) as u64) << 2;
            imm |= (((raw >> 6) & 1) as u64) << 4;
            imm |= (((raw >> 12) & 1) as u64) << 5;
            if rd != 0 {
                Instr::Lw {
                    rd,
                    rs1: 2,
                    imm: imm as i64,
                }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (2, 3) => {
            // c.ldsp
            let rd = ((raw >> 7) & 0x1f) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 4) & 3) as u64) << 3;
            imm |= (((raw >> 12) & 1) as u64) << 5;
            imm |= (((raw >> 2) & 3) as u64) << 6;
            imm |= (((raw >> 6) & 1) as u64) << 4;
            if rd != 0 {
                Instr::Ld {
                    rd,
                    rs1: 2,
                    imm: imm as i64,
                }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        (2, 6) => {
            // c.swsp
            let rs2 = ((raw >> 2) & 0x1f) as u8;
            let mut imm = 0u64;
            if (raw >> 9) & 1 != 0 {
                imm |= 4;
            }
            if (raw >> 3) & 1 != 0 {
                imm |= 8;
            }
            if (raw >> 10) & 1 != 0 {
                imm |= 4;
            }
            Instr::Sw {
                rs1: 2,
                rs2,
                imm: imm as i64,
            }
        }
        (2, 7) => {
            // c.sdsp
            let rs2 = ((raw >> 2) & 0x1f) as u8;
            let mut imm = 0u64;
            imm |= (((raw >> 4) & 3) as u64) << 3;
            imm |= (((raw >> 12) & 1) as u64) << 5;
            imm |= (((raw >> 2) & 3) as u64) << 6;
            imm |= (((raw >> 6) & 1) as u64) << 4;
            Instr::Sd {
                rs1: 2,
                rs2,
                imm: imm as i64,
            }
        }
        (2, 4) => {
            let rd = ((raw >> 7) & 0x1f) as u8;
            let rs2 = ((raw >> 2) & 0x1f) as u8;
            let bit12 = (raw >> 12) & 1;
            if bit12 == 0 && rs2 == 0 && rd != 0 {
                Instr::Jalr {
                    rd: 0,
                    rs1: rd,
                    imm: 0,
                }
            } else if bit12 == 0 && rs2 != 0 && rd != 0 {
                // c.mv rd, rs2
                Instr::Add { rd, rs1: 0, rs2 }
            } else if bit12 == 1 && rs2 == 0 && rd != 0 {
                Instr::Jalr {
                    rd: 1,
                    rs1: rd,
                    imm: 0,
                }
            } else if bit12 == 1 && rs2 != 0 && rd != 0 {
                // c.add rd, rd, rs2
                Instr::Add { rd, rs1: rd, rs2 }
            } else {
                Instr::Unknown(raw as u32)
            }
        }
        _ => Instr::Unknown(raw as u32),
    }
}
