void _start(void) {
    float fa0 = 1.0f;
    float fa1 = 41.0f;
    float fa2 = fa0 + fa1;
    int a0 = (int)fa2;
    __asm__ volatile (
        "li a7, 93\n"
        "mv a0, %0\n"
        "ecall\n"
        :: "r"(a0)
        : "a0", "a7"
    );
}
