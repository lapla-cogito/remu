pub enum TcgOpcode {
    MovI64,
    AddI64,
    SubI64,
    AndI64,
    OrI64,
    XorI64,
    ShlI64,
    ShrI64,
    SarI64,
    SetCondI64,
    QemuLdI64,
    QemuStI64,
    SetLabel,
    Br,
    BrCondI64,
    ExitTb,
    Call,
}

pub enum TcgArg {
    Temp(u32),
    Const(u64),
    Label(u32),
}

pub struct TcgOp {
    pub opc: TcgOpcode,
    pub args: smallvec::SmallVec<[TcgArg; 4]>,
}
