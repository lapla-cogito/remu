void _start(void) {
    int t0 = 5;
    int t6 = t0 << 1;
    int t1 = (t0 < 10);
    int t2 = ((unsigned)t0 < 10);
    int t3 = t0 & 3;
    int t4 = t0 | 0xa;
    int t5 = t0 ^ 0xf;
    if (t0 < 3) {
        __asm__ volatile ("li a0,99; li a7,93; ecall");
    }
    int a0 = 6 * 7;
    const char *m = "OK\n";
    __asm__ volatile (
        "li a7,64; li a0,1; mv a1,%0; li a2,3; ecall\n"
        "li a7,93; li a0,42; ecall\n"
        :: "r"(m) : "a0","a1","a2","a7","memory"
    );
}
