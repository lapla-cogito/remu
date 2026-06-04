void _start(void) {
    const char *msg = "Hello RV64\n";
    __asm__ volatile (
        "li a7, 64\n"
        "li a0, 1\n"
        "mv a1, %0\n"
        "li a2, 11\n"
        "ecall\n"
        :: "r"(msg)
        : "a0","a1","a2","a7","memory"
    );
    __asm__ volatile (
        "li a7, 93\n"
        "li a0, 42\n"
        "ecall\n"
    );
}
