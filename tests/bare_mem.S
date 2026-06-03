	.text
	.global _start
_start:
	la t0, data
	lb t1, 0(t0)
	lbu t2, 0(t0)
	lh t3, 2(t0)
	lhu t4, 2(t0)
	lw t5, 4(t0)
	lwu t6, 4(t0)
	ld a0, 8(t0)
.Lout:
	auipc t0, %pcrel_hi(outbuf)
	addi t0, t0, %pcrel_lo(.Lout)
	li t1, 'M'
	sb t1, 0(t0)
	li t1, 'E'
	sb t1, 1(t0)
	li t1, 'M'
	sb t1, 2(t0)
	li t1, 'T'
	sb t1, 3(t0)
	li t1, 'E'
	sb t1, 4(t0)
	li t1, 'S'
	sb t1, 5(t0)
	li t1, 'T'
	sb t1, 6(t0)
	li t1, '\n'
	sb t1, 7(t0)
	li a0, 1
	mv a1, t0
	li a2, 8
	li a7, 64
	ecall
	li a0, 42
	li a7, 93
	ecall
data:
	.byte 0x80, 0x7f, 0x00, 0xff, 0x34, 0x12, 0x00, 0x00
	.quad 0x1234567890abcdef
	.data
outbuf:
	.space 16
