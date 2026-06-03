#[derive(Clone, Copy)]
pub enum TcgOpcode {
    #[expect(dead_code)]
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
    GetGprI64,
    SetGprI64,
    QemuLdI64,
    QemuStI64,
    SetNextPcI64,
    SetLabel,
    #[expect(dead_code)]
    Br,
    BrCondI64,
    ExitTb,
    Call,
}

#[derive(Clone, Copy)]
pub enum TcgArg {
    Temp(u32),
    Const(u64),
    Label(u32),
}

pub struct TcgOp {
    pub opc: TcgOpcode,
    pub args: smallvec::SmallVec<[TcgArg; 4]>,
}
