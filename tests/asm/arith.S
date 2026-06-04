	.text
	.global _start
_start:
	li t0, 5
	slliw t6, t0, 1
	slti t1, t0, 10
	sltiu t2, t0, 10
	andi t3, t0, 3
	ori t4, t0, 0xa
	xori t5, t0, 0xf
	li t0, 5
	li t1, 3
	bge t0, t1, .Lyes
	li a0, 99
	li a7, 93
	ecall
.Lyes:
	li a0, 1
	la a1, msg
	li a2, 3
	li a7, 64
	ecall
	li a0, 6
	li t1, 7
	mul a0, a0, t1
	li a7, 93
	ecall
msg:
	.ascii "OK\n"
