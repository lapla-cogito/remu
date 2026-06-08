	.text
	.global _start
_start:
	la t0, target
	csrw mepc, t0
	li t1, (1 << 11)
	csrs mstatus, t1
	mret
target:
	li a0, 42
	li a7, 93
	ecall
