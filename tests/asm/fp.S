.global _start
_start:
    auipc t0, 0
    addi t0, t0, 32
    flw fa0, 0(t0)
    flw fa1, 4(t0)
    fadd.s fa2, fa0, fa1
    fcvt.w.s a0, fa2
    li a7, 93
    ecall
.align 4
.float 1.0
.float 41.0
