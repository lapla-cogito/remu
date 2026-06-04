#include "syscalls.h"

void _start(void) {
    float fa0 = 1.0f;
    float fa1 = 41.0f;
    float fa2 = fa0 + fa1;
    int a0 = (int)fa2;
    sys_exit(a0);
}
