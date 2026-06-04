	.text
	.global _start
_start:
	li a0, 1
	la a1, msg
	li a2, 11
	li a7, 64
	ecall
	li a0, 42
	li a7, 93
	ecall
msg:
	.ascii "Hello RV64\n"
