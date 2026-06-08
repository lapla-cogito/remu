.text
.global _start
_start:
    la t0, handler
    csrw mtvec, t0
    ecall
    li a0, 1
    li a7, 93
    ecall
handler:
    li t0, 0
    csrw mtvec, t0
    li a0, 42
    li a7, 93
    ecall
