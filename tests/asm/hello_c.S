	.text
	.global _start
_start:
	c.li a0, 1
	c.li a2, 11
	c.mv a1, a2
	c.add a2, a0
	c.sub a2, a0
	c.and a2, a2
.Laddr:
	auipc a1, %pcrel_hi(msg)
	addi a1, a1, %pcrel_lo(.Laddr)
	c.addi a1, 0
	li a7, 64
	ecall
	c.li a0, 31
	c.addi a0, 11
	li a7, 93
	ecall
msg:
	.ascii "Hello RV64\n"
