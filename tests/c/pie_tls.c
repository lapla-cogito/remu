#include "syscalls.h"

__thread int tls_value = 42;
static int global_data = 99;
int *ptr_to_global = &global_data;

void _start(void) {
    /* Access TLS and global pointer to pull PT_TLS (and potential relocs) in PIE build. */
    int t = tls_value;
    int g = *ptr_to_global;
    (void)t;
    (void)g;
    sys_write(1, "PIETLSOK\n", 9);
    sys_exit(42);
}
