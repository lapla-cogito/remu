	.text
	.global _start
_start:
	la t0, data
	li t1, 0
	sw t1, 0(t0)
	lr.w t2, (t0)
	li t3, 42
	sc.w t4, t3, (t0)
	lw t5, 0(t0)
	# amoadd
	li t1, 0
	sw t1, 0(t0)
	amoadd.w t6, t5, (t0)  # old 0 to t6, +42 -> mem 42
	lw a0, 0(t0)
	li a7, 93
	ecall
	.data
data:
	.word 0
	.quad 0
