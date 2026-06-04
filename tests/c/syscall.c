#include "syscalls.h"

char msg_fstat[] = "FSTAT\n";
char msg_wv[] = "WV\n";
char msg_ioctl[] = "IOCTL\n";
char msg_un[] = "UN\n";
char msg_brk[] = "BRK\n";
char msg_sysok[] = "SYSOK\n";
char s1[3] = "WV";
char s2[4] = "OK\n";
struct iov_t iovs[2];

void _start(void) {
    __asm__ volatile (
        "la gp, __global_pointer$\n"
        ::: "gp"
    );
    unsigned char statbuf[128];
    long r = sys_fstat(1, statbuf);
    (void)r;
    sys_write(1, msg_fstat, 6);

    iovs[0].base = s1; iovs[0].len = 2;
    iovs[1].base = s2; iovs[1].len = 3;
    sys_writev(1, iovs, 2);
    sys_write(1, msg_wv, 3);

    unsigned char term[60];
    sys_ioctl(1, 0x5401, term);
    sys_write(1, msg_ioctl, 6);

    unsigned char uts[65 * 6];
    sys_uname(uts);
    sys_write(1, msg_un, 3);

    unsigned long cur = sys_brk(0);
    unsigned long nb = cur + 0x1000;
    sys_brk(nb);
    char *p = (char *) cur;
    p[0] = 'B'; p[1] = 'R'; p[2] = 'K'; p[3] = 'U'; p[4] = 'S'; p[5] = 'E'; p[6] = '\n';
    sys_write(1, p, 7);

    sys_openat(-100, 0, 0, 0);
    sys_close(100);

    sys_mmap(0, 0x1000, 3, 0x22, -1, 0);

    char pth[] = "/proc/self/exe";
    char lbuf[16];
    sys_readlinkat(-100, pth, lbuf, sizeof(lbuf));
    sys_write(1, "RLINK\n", 6);

    unsigned char rbuf[8];
    sys_getrandom(rbuf, sizeof(rbuf), 0);
    sys_write(1, "GRND\n", 5);

    sys_prlimit64(0, 0, 0, 0);
    sys_write(1, "PRL\n", 4);

    sys_getpid();
    sys_set_tid_address(0);
    sys_set_robust_list(0, 0);
    sys_mprotect(0, 0, 0);
    sys_write(1, "MORE\n", 5);

    sys_write(1, msg_sysok, 6);
    sys_exit(42);
}
