# remu

RISC-V rv64gc emulator in Rust with a QEMU-style Tiny Code Generator (TCG).

## Features

- rv64gc (I+M+A+F+D+C)
- Static ELF loading (`PT_LOAD`, `__global_pointer$`, minimal stack/auxv)
- Execution modes:
  - `interp`: straightforward interpreter
  - `tcg-interp`: translate blocks to TCG IR and interpret
  - `jit`: translate to TCG then compile to native code
- Basic syscalls for simple programs and limited glibc-static C (write, brk, fstat, writev, mmap, etc.)
- TB caching, optional tracing (`--trace`)

## Usage

```sh
remu [OPTIONS] <ELF>
```

- `--mode <interp|tcg-interp|jit>`: select execution mode (default: `interp`)
- `--trace`: emit in_asm / op / out_asm style logs

The `<ELF>` must be a static rv64gc Linux binary.

Example:

```sh
remu --mode jit /path/to/bin
```

## Testing

```sh
cargo test
```

Tests compile the programs under `tests/asm/` and `tests/c/` on the fly and assert that all three modes produce the same stdout and exit status as `qemu-riscv64`. You also need the RISC-V GNU toolchain and QEMU user emulation (`riscv64-linux-gnu-gcc`, `qemu-riscv64`, etc.).
