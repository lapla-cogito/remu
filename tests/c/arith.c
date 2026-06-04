#include "syscalls.h"

void _start(void) {
    int t0 = 5;
    int t6 = t0 << 1;
    int t1 = (t0 < 10);
    int t2 = ((unsigned)t0 < 10);
    int t3 = t0 & 3;
    int t4 = t0 | 0xa;
    int t5 = t0 ^ 0xf;
    if (t0 < 3) {
        sys_exit(99);
    }
    int a0 = 6 * 7;
    const char *m = "OK\n";
    sys_write(1, m, 3);
    sys_exit(42);
}
