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

    pub fn gen_mul_i64(
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
            opc: crate::tcg::op::TcgOpcode::MulI64,
            args,
        });
    }

    pub fn gen_mulh_i64(
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
            opc: crate::tcg::op::TcgOpcode::MulhI64,
            args,
        });
    }

    pub fn gen_mulhsu_i64(
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
            opc: crate::tcg::op::TcgOpcode::MulhsuI64,
            args,
        });
    }

    pub fn gen_mulhu_i64(
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
            opc: crate::tcg::op::TcgOpcode::MulhuI64,
            args,
        });
    }

    pub fn gen_divs_i64(
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
            opc: crate::tcg::op::TcgOpcode::DivsI64,
            args,
        });
    }

    pub fn gen_divu_i64(
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
            opc: crate::tcg::op::TcgOpcode::DivuI64,
            args,
        });
    }

    pub fn gen_rems_i64(
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
            opc: crate::tcg::op::TcgOpcode::RemsI64,
            args,
        });
    }

    pub fn gen_remu_i64(
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
            opc: crate::tcg::op::TcgOpcode::RemuI64,
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

    pub fn gen_qemu_ld8_signed(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLd8Signed,
            args,
        });
    }

    pub fn gen_qemu_ld8_unsigned(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLd8Unsigned,
            args,
        });
    }

    pub fn gen_qemu_ld16_signed(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLd16Signed,
            args,
        });
    }

    pub fn gen_qemu_ld16_unsigned(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLd16Unsigned,
            args,
        });
    }

    pub fn gen_qemu_ld32_signed(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLd32Signed,
            args,
        });
    }

    pub fn gen_qemu_ld32_unsigned(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLd32Unsigned,
            args,
        });
    }

    pub fn gen_qemu_ld64(&mut self, dst: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuLd64,
            args,
        });
    }

    pub fn gen_qemu_st8(&mut self, src: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(src);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuSt8,
            args,
        });
    }

    pub fn gen_qemu_st16(&mut self, src: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(src);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuSt16,
            args,
        });
    }

    pub fn gen_qemu_st32(&mut self, src: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(src);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuSt32,
            args,
        });
    }

    pub fn gen_qemu_st64(&mut self, src: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(src);
        args.push(addr);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::QemuSt64,
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

    pub fn gen_lr_w(&mut self, dst: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        args.push(crate::tcg::op::TcgArg::Const(0));
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::LrW,
            args,
        });
    }

    pub fn gen_lr_d(&mut self, dst: crate::tcg::op::TcgArg, addr: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        args.push(crate::tcg::op::TcgArg::Const(0));
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::LrD,
            args,
        });
    }

    pub fn gen_sc_w(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
        val: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        args.push(val);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::ScW,
            args,
        });
    }

    pub fn gen_sc_d(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        addr: crate::tcg::op::TcgArg,
        val: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(addr);
        args.push(val);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::ScD,
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

    pub fn gen_get_fpr_i64(&mut self, dst: crate::tcg::op::TcgArg, reg: u8) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(crate::tcg::op::TcgArg::Const(reg as u64));
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::GetFprI64,
            args,
        });
    }

    pub fn gen_set_fpr_i64(&mut self, reg: u8, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(crate::tcg::op::TcgArg::Const(reg as u64));
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::SetFprI64,
            args,
        });
    }

    pub fn gen_fadd_s(
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
            opc: crate::tcg::op::TcgOpcode::FAddS,
            args,
        });
    }

    pub fn gen_fadd_d(
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
            opc: crate::tcg::op::TcgOpcode::FAddD,
            args,
        });
    }

    pub fn gen_fsub_s(
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
            opc: crate::tcg::op::TcgOpcode::FSubS,
            args,
        });
    }

    pub fn gen_fsub_d(
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
            opc: crate::tcg::op::TcgOpcode::FSubD,
            args,
        });
    }

    pub fn gen_fmul_s(
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
            opc: crate::tcg::op::TcgOpcode::FMulS,
            args,
        });
    }

    pub fn gen_fmul_d(
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
            opc: crate::tcg::op::TcgOpcode::FMulD,
            args,
        });
    }

    pub fn gen_fdiv_s(
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
            opc: crate::tcg::op::TcgOpcode::FDivS,
            args,
        });
    }

    pub fn gen_fdiv_d(
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
            opc: crate::tcg::op::TcgOpcode::FDivD,
            args,
        });
    }

    pub fn gen_fsqrt_s(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FSqrtS,
            args,
        });
    }

    pub fn gen_fsqrt_d(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FSqrtD,
            args,
        });
    }

    pub fn gen_fmadd_s(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMaddS,
            args,
        });
    }

    pub fn gen_fmadd_d(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMaddD,
            args,
        });
    }

    pub fn gen_fmsub_s(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMsubS,
            args,
        });
    }

    pub fn gen_fmsub_d(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMsubD,
            args,
        });
    }

    pub fn gen_fnmadd_s(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FNmaddS,
            args,
        });
    }

    pub fn gen_fnmadd_d(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FNmaddD,
            args,
        });
    }

    pub fn gen_fnmsub_s(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FNmsubS,
            args,
        });
    }

    pub fn gen_fnmsub_d(
        &mut self,
        dst: crate::tcg::op::TcgArg,
        s1: crate::tcg::op::TcgArg,
        s2: crate::tcg::op::TcgArg,
        s3: crate::tcg::op::TcgArg,
    ) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(s1);
        args.push(s2);
        args.push(s3);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FNmsubD,
            args,
        });
    }

    pub fn gen_fcvt_ws(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtWS,
            args,
        });
    }

    pub fn gen_fcvt_wus(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtWUS,
            args,
        });
    }

    pub fn gen_fcvt_ls(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtLS,
            args,
        });
    }

    pub fn gen_fcvt_lus(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtLUS,
            args,
        });
    }

    pub fn gen_fcvt_sw(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtSW,
            args,
        });
    }

    pub fn gen_fcvt_sl(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtSL,
            args,
        });
    }

    pub fn gen_fcvt_swu(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtSWU,
            args,
        });
    }

    pub fn gen_fcvt_slu(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtSLU,
            args,
        });
    }

    pub fn gen_fcvt_wd(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtWD,
            args,
        });
    }

    pub fn gen_fcvt_wud(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtWUD,
            args,
        });
    }

    pub fn gen_fcvt_ld(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtLD,
            args,
        });
    }

    pub fn gen_fcvt_lud(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtLUD,
            args,
        });
    }

    pub fn gen_fcvt_dw(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtDW,
            args,
        });
    }

    pub fn gen_fcvt_dl(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtDL,
            args,
        });
    }

    pub fn gen_fcvt_dwu(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtDWU,
            args,
        });
    }

    pub fn gen_fcvt_dlu(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtDLU,
            args,
        });
    }

    pub fn gen_fcvt_sd(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtSD,
            args,
        });
    }

    pub fn gen_fcvt_ds(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FCvtDS,
            args,
        });
    }

    pub fn gen_fmv_xw(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMvXW,
            args,
        });
    }

    pub fn gen_fmv_wx(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMvWX,
            args,
        });
    }

    #[expect(dead_code)]
    pub fn gen_fmv_xd(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMvXD,
            args,
        });
    }

    #[expect(dead_code)]
    pub fn gen_fmv_dx(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FMvDX,
            args,
        });
    }

    pub fn gen_fsgnj_s(
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
            opc: crate::tcg::op::TcgOpcode::FSgnjS,
            args,
        });
    }

    pub fn gen_fsgnj_d(
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
            opc: crate::tcg::op::TcgOpcode::FSgnjD,
            args,
        });
    }

    pub fn gen_fsgnjn_s(
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
            opc: crate::tcg::op::TcgOpcode::FSgnjnS,
            args,
        });
    }

    pub fn gen_fsgnjn_d(
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
            opc: crate::tcg::op::TcgOpcode::FSgnjnD,
            args,
        });
    }

    pub fn gen_fsgnjx_s(
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
            opc: crate::tcg::op::TcgOpcode::FSgnjxS,
            args,
        });
    }

    pub fn gen_fsgnjx_d(
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
            opc: crate::tcg::op::TcgOpcode::FSgnjxD,
            args,
        });
    }

    pub fn gen_fmin_s(
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
            opc: crate::tcg::op::TcgOpcode::FMinS,
            args,
        });
    }

    pub fn gen_fmin_d(
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
            opc: crate::tcg::op::TcgOpcode::FMinD,
            args,
        });
    }

    pub fn gen_fmax_s(
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
            opc: crate::tcg::op::TcgOpcode::FMaxS,
            args,
        });
    }

    pub fn gen_fmax_d(
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
            opc: crate::tcg::op::TcgOpcode::FMaxD,
            args,
        });
    }

    pub fn gen_feq_s(
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
            opc: crate::tcg::op::TcgOpcode::FeqS,
            args,
        });
    }

    pub fn gen_feq_d(
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
            opc: crate::tcg::op::TcgOpcode::FeqD,
            args,
        });
    }

    pub fn gen_flt_s(
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
            opc: crate::tcg::op::TcgOpcode::FltS,
            args,
        });
    }

    pub fn gen_flt_d(
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
            opc: crate::tcg::op::TcgOpcode::FltD,
            args,
        });
    }

    pub fn gen_fle_s(
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
            opc: crate::tcg::op::TcgOpcode::FleS,
            args,
        });
    }

    pub fn gen_fle_d(
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
            opc: crate::tcg::op::TcgOpcode::FleD,
            args,
        });
    }

    pub fn gen_fclass_s(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FClassS,
            args,
        });
    }

    pub fn gen_fclass_d(&mut self, dst: crate::tcg::op::TcgArg, src: crate::tcg::op::TcgArg) {
        let mut args = smallvec::SmallVec::new();
        args.push(dst);
        args.push(src);
        self.ops.push(crate::tcg::op::TcgOp {
            opc: crate::tcg::op::TcgOpcode::FClassD,
            args,
        });
    }
}
