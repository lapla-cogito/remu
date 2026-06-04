#include "syscalls.h"

void _start(void) {
    const char *msg = "Hello RV64\n";
    sys_write(1, msg, 11);
    sys_exit(42);
}
