pub struct TcgContext {
    pub ops: Vec<crate::tcg::op::TcgOp>,
    next_temp: u32,
    next_label: u32,
}

impl TcgContext {
    pub fn num_temps(&self) -> u32 {
        self.next_temp
    }

    pub fn num_labels(&self) -> u32 {
        self.next_label
    }
}

impl TcgContext {
    pub fn new() -> Self {
        TcgContext {
            ops: Vec::new(),
            next_temp: 0,
            next_label: 0,
        }
    }

    pub fn new_temp(&mut self) -> crate::tcg::op::TcgArg {
        let t = self.next_temp;
        self.next_temp += 1;
        crate::tcg::op::TcgArg::Temp(t)
    }

    pub fn new_const(&self, val: u64) -> crate::tcg::op::TcgArg {
        crate::tcg::op::TcgArg::Const(val)
    }

    pub fn new_label(&mut self) -> crate::tcg::op::TcgArg {
        let l = self.next_label;
        self.next_label += 1;
        crate::tcg::op::TcgArg::Label(l)
    }

    #[expect(dead_code)]
    pub fn gen_mov_i64(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::MovI64,
            args,
        });
    }

    pub fn gen_add_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::AddI64,
            args,
        });
    }

    pub fn gen_sub_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::SubI64,
            args,
        });
    }

    pub fn gen_and_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::AndI64,
            args,
        });
    }

    pub fn gen_or_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::OrI64,
            args,
        });
    }

    pub fn gen_xor_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::XorI64,
            args,
        });
    }

    pub fn gen_shl_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::ShlI64,
            args,
        });
    }

    pub fn gen_shr_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::ShrI64,
            args,
        });
    }

    pub fn gen_sar_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::SarI64,
            args,
        });
    }

    pub fn gen_set_cond_i64(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        cond: u32,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(crate::tcg::op::TcgArg::Const(cond as u64));
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::SetCondI64,
            args,
        });
    }

    pub fn gen_qemu_ld_i64(&mut self, dst: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLdI64,
            args,
        });
    }

    pub fn gen_qemu_st_i64(&mut self, src: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(src);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuStI64,
            args,
        });
    }

    pub fn gen_set_next_pc(&mut self, target: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(target);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::SetNextPcI64,
            args,
        });
    }

    pub fn gen_set_label(&mut self, label: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(label);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::SetLabel,
            args,
        });
    }

    #[expect(dead_code)]
    pub fn gen_br(&mut self, label: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(label);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::Br,
            args,
        });
    }

    pub fn gen_brcond_i64(
        &mut self,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        cond: u32,
        label: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(s1);
        args.push(s2);
        args.push(crate::tcg::op::TcgArg::Const(cond as u64));
        args.push(label);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::BrCondI64,
            args,
        });
    }

    pub fn gen_exit_tb(&mut self) {
        let args = smallvec::SmallVec::new();
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::ExitTb,
            args,
        });
    }

    pub fn gen_call(&mut self, helper: u64, num_args: u32) {
        let mut args = smallvec::SmallVec::new();
        args.push(crate::tcg::op::TcgArg::Const(helper));
        args.push(crate::tcg::op::TcgArg::Const(num_args as u64));
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::Call,
            args,
        });
    }

    pub fn gen_get_gpr_i64(&mut self, dst: crate::tcg::op::TcgArg, reg: u8) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(crate::tcg::op::TcgArg::Const(reg as u64));
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::GetGprI64,
            args,
        });
    }

    pub fn gen_set_gpr_i64(&mut self, reg: u8, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(crate::tcg::op::TcgArg::Const(reg as u64));
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::SetGprI64,
            args,
        });
    }
}
