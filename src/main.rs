mod cpu;
mod decode;
mod elf_loader;
mod interp;
mod memory;
mod syscall;
mod tcg;

type TcgTbCache = hashbrown::HashMap<u64, (crate::tcg::context::TcgContext, u64)>;
type JitTbCache = hashbrown::HashMap<u64, (dynasmrt::ExecutableBuffer, u64)>;

#[derive(clap::Parser)]
#[command(
    version,
    about = "remu - RISC-V rv64gc emulator with TCG (interp/tcg-interp/jit modes)"
)]
struct Args {
    #[arg(long, default_value = "interp")]
    mode: String,
    #[arg(long)]
    trace: bool,
    elf: String,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    guest_args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = <Args as clap::Parser>::parse();
    let mut cpu = crate::cpu::Cpu::new(0);
    let mut mem = crate::memory::GuestMemory::new();
    crate::elf_loader::load_elf(&args.elf, &mut cpu, &mut mem)?;
    if args.trace {
        println!("trace: entry {:#x}", cpu.pc);
    }
    let mut steps: u64 = 0;
    let mut tcg_cache: TcgTbCache = hashbrown::HashMap::new();
    let mut jit_cache: JitTbCache = hashbrown::HashMap::new();
    loop {
        if args.mode == "interp" {
            crate::interp::step(&mut cpu, &mut mem)?;
        } else if args.mode == "tcg-interp" {
            let block_start = cpu.pc;
            if args.trace {
                println!("trace: TB {:#x}", block_start);
            }
            let (ctx, end_pc) = if let Some(entry) = tcg_cache.get(&block_start) {
                (&entry.0, entry.1)
            } else {
                let translated =
                    crate::tcg::frontend::translate_block(block_start, &mem, 64, args.trace);
                tcg_cache.insert(block_start, translated);
                let entry = tcg_cache.get(&block_start).expect("just inserted");
                (&entry.0, entry.1)
            };
            let next_pc = crate::tcg::backend::execute_tcg(ctx, &mut cpu, &mut mem);
            cpu.pc = next_pc.unwrap_or(end_pc);
            if cpu.pc == block_start {
                return Ok(());
            }
        } else if args.mode == "jit" {
            let block_start = cpu.pc;
            if args.trace {
                println!("trace: TB {:#x}", block_start);
            }
            let buf = if let Some(entry) = jit_cache.get(&block_start) {
                &entry.0
            } else {
                let translated =
                    crate::tcg::frontend::translate_block(block_start, &mem, 64, args.trace);
                let b = crate::tcg::jit::compile(&translated.0, translated.1, args.trace)?;
                jit_cache.insert(block_start, (b, translated.1));
                let entry = jit_cache.get(&block_start).expect("just inserted");
                &entry.0
            };
            let f: extern "C" fn(*mut u64, *mut u64, *mut u8) -> u64 =
                unsafe { std::mem::transmute(buf.as_ptr()) };
            let gpr = cpu.gpr.as_mut_ptr();
            let fpr = cpu.fpr.as_mut_ptr();
            let mem_base = mem.mem_ptr();
            let next_pc = f(gpr, fpr, mem_base);
            cpu.pc = next_pc;
            if cpu.pc == block_start {
                return Ok(());
            }
        } else {
            anyhow::bail!("only interp, tcg-interp and jit modes supported");
        }
        steps += 1;
        if steps > 10_000_000 {
            anyhow::bail!("step limit");
        }
    }
}

#[cfg(test)]
mod tests {
    fn compile_assemble(src_path: &str, out_elf: &str) -> String {
        if src_path.ends_with(".c") {
            let s = std::process::Command::new("riscv64-linux-gnu-gcc")
                .args([
                    "-march=rv64gc",
                    "-mabi=lp64d",
                    "-nostdlib",
                    "-nostartfiles",
                    "-O1",
                    "-fomit-frame-pointer",
                    "-static",
                    "-I",
                    "tests/c",
                    "-o",
                    out_elf,
                    src_path,
                ])
                .status()
                .expect("gcc");
            if !s.success() {
                panic!("gcc failed for {}", src_path);
            }
        } else {
            let obj = std::format!("{}.o", out_elf);
            let s = std::process::Command::new("riscv64-linux-gnu-as")
                .args(["-march=rv64gc", src_path, "-o", &obj])
                .status()
                .expect("as");
            if !s.success() {
                panic!("as failed for {}", src_path);
            }
            let s = std::process::Command::new("riscv64-linux-gnu-ld")
                .args(["-o", out_elf, &obj])
                .status()
                .expect("ld");
            if !s.success() {
                panic!("ld failed for {}", out_elf);
            }
        }
        out_elf.to_string()
    }

    fn run_capture(elf: &str, mode: &str) -> std::process::Output {
        if mode == "qemu" {
            std::process::Command::new("qemu-riscv64")
                .arg(elf)
                .output()
                .expect("qemu")
        } else {
            std::process::Command::new("./target/debug/remu")
                .args(["--mode", mode, elf])
                .output()
                .expect(mode)
        }
    }

    fn assert_all_modes_match(src: &str, exp_code: i32, exp_stdout: &[u8]) {
        let name = src.rsplit('/').next().unwrap_or(src);
        let stem = name.trim_end_matches(".S").trim_end_matches(".c");
        let elf = compile_assemble(src, &std::format!("/tmp/{}", stem));
        let modes = ["interp", "tcg-interp", "jit", "qemu"];
        let reference = run_capture(&elf, modes[0]);
        for &mode in &modes[1..] {
            let out = run_capture(&elf, mode);
            assert_eq!(reference.status, out.status, "status differ for {}", mode);
            assert_eq!(reference.stdout, out.stdout, "stdout differ for {}", mode);
        }
        assert_eq!(reference.status.code(), Some(exp_code));
        assert_eq!(reference.stdout.as_slice(), exp_stdout);
    }

    #[test]
    fn hello_matches_all_modes() {
        assert_all_modes_match("tests/asm/hello.S", 42, b"Hello RV64\n");
    }

    #[test]
    fn hello_c_matches_all_modes() {
        assert_all_modes_match("tests/asm/hello_c.S", 42, b"Hello RV64\n");
    }

    #[test]
    fn mem_matches_all_modes() {
        assert_all_modes_match("tests/asm/mem.S", 42, b"MEMTEST\n");
    }

    #[test]
    fn arith_and_branches_match_all_modes() {
        assert_all_modes_match("tests/asm/arith.S", 42, b"OK\n");
    }

    #[test]
    fn fp_matches_all_modes() {
        assert_all_modes_match("tests/asm/fp.S", 42, b"");
    }

    #[test]
    fn c_hello_matches_all_modes() {
        assert_all_modes_match("tests/c/hello.c", 42, b"Hello RV64\n");
    }

    #[test]
    fn c_arith_matches_all_modes() {
        assert_all_modes_match("tests/c/arith.c", 42, b"OK\n");
    }

    #[test]
    fn c_fp_matches_all_modes() {
        assert_all_modes_match("tests/c/fp.c", 42, b"");
    }

    #[test]
    fn atomic_matches_all_modes() {
        assert_all_modes_match("tests/asm/atomic.S", 42, b"");
    }

    #[test]
    fn c_syscall_matches_all_modes() {
        assert_all_modes_match(
            "tests/c/syscall.c",
            42,
            b"FSTAT\nWVOK\nWV\nIOCTL\nUN\nBRKUSE\nRLINK\nGRND\nPRL\nMORE\nSYSOK\n",
        );
    }
}
