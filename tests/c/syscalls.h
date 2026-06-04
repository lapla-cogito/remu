struct iov_t { void* base; unsigned long len; };

static long _sys(long n, long a0, long a1, long a2, long a3, long a4, long a5) {
    register long a7 asm("a7") = n;
    register long x10 asm("a0") = a0;
    register long x11 asm("a1") = a1;
    register long x12 asm("a2") = a2;
    register long x13 asm("a3") = a3;
    register long x14 asm("a4") = a4;
    register long x15 asm("a5") = a5;
    asm volatile("ecall"
        : "+r"(x10)
        : "r"(x11), "r"(x12), "r"(x13), "r"(x14), "r"(x15), "r"(a7)
        : "memory");
    return x10;
}
static long sys_fstat(int fd, void *buf) { return _sys(80, fd, (long)buf, 0,0,0,0); }
static long sys_write(int fd, const void *buf, long len) { return _sys(64, fd, (long)buf, len,0,0,0); }
static long sys_writev(int fd, const void *iov, long iovcnt) { return _sys(66, fd, (long)iov, iovcnt,0,0,0); }
static long sys_ioctl(int fd, long cmd, void *arg) { return _sys(29, fd, cmd, (long)arg,0,0,0); }
static long sys_uname(void *buf) { return _sys(160, (long)buf, 0,0,0,0,0); }
static long sys_brk(unsigned long addr) { return _sys(214, addr, 0,0,0,0,0); }
static long sys_openat(int dirfd, const void *path, long flags, long mode) { return _sys(56, dirfd, (long)path, flags, mode,0,0); }
static long sys_close(int fd) { return _sys(57, fd, 0,0,0,0,0); }
static long sys_mmap(unsigned long addr, unsigned long len, long prot, long flags, long fd, long off) { return _sys(222, addr, len, prot, flags, fd, off); }
static long sys_readlinkat(int dirfd, const void *path, void *buf, long bufsiz) { return _sys(78, dirfd, (long)path, (long)buf, bufsiz,0,0); }
static long sys_getrandom(void *buf, long len, long flags) { return _sys(278, (long)buf, len, flags,0,0,0); }
static long sys_prlimit64(long pid, long res, void *newp, void *oldp) { return _sys(261, pid, res, (long)newp, (long)oldp,0,0); }
static long sys_set_tid_address(void *ptr) { return _sys(96, (long)ptr,0,0,0,0,0); }
static long sys_set_robust_list(void *head, long len) { return _sys(99, (long)head, len,0,0,0,0); }
static long sys_mprotect(void *addr, long len, long prot) { return _sys(226, (long)addr, len, prot,0,0,0); }
static long sys_getpid(void) { return _sys(172,0,0,0,0,0,0); }
static void sys_exit(int code) { _sys(93, code, 0,0,0,0,0); }
