mod cpu;
mod decode;
mod elf_loader;
mod interp;
mod memory;
mod syscall;
mod tcg;

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
    loop {
        if args.mode == "interp" {
            crate::interp::step(&mut cpu, &mut mem)?;
        } else if args.mode == "tcg-interp" {
            let block_start = cpu.pc;
            let (ctx, end_pc) = crate::tcg::frontend::translate_block(block_start, &mem, 64);
            let next_pc = crate::tcg::backend::execute_tcg(&ctx, &mut cpu, &mut mem);
            cpu.pc = next_pc.unwrap_or(end_pc);
            if cpu.pc == block_start {
                return Ok(());
            }
            // If ended at unhandled control (branch), execute it with interp to update pc
            if let Ok((_, last)) = crate::decode::fetch_decode(&mem, cpu.pc)
                && matches!(
                    last,
                    crate::decode::Instr::Beq { .. }
                        | crate::decode::Instr::Bne { .. }
                        | crate::decode::Instr::Blt { .. }
                        | crate::decode::Instr::Bge { .. }
                        | crate::decode::Instr::Bltu { .. }
                        | crate::decode::Instr::Bgeu { .. }
                )
            {
                crate::interp::step(&mut cpu, &mut mem)?;
            }
        } else if args.mode == "jit" {
            let block_start = cpu.pc;
            let (ctx, end_pc) = crate::tcg::frontend::translate_block(block_start, &mem, 64);
            let buf = crate::tcg::jit::compile(&ctx)?;
            let f: extern "C" fn(*mut u64, *mut u64, *mut u8) =
                unsafe { std::mem::transmute(buf.as_ptr()) };
            let gpr = cpu.gpr.as_mut_ptr();
            let fpr = cpu.fpr.as_mut_ptr();
            let mem_base = mem.mem_ptr();
            f(gpr, fpr, mem_base);
            cpu.pc = end_pc;
            if cpu.pc == block_start {
                return Ok(());
            }
            if let Ok((_, last)) = crate::decode::fetch_decode(&mem, cpu.pc)
                && matches!(
                    last,
                    crate::decode::Instr::Beq { .. }
                        | crate::decode::Instr::Bne { .. }
                        | crate::decode::Instr::Blt { .. }
                        | crate::decode::Instr::Bge { .. }
                        | crate::decode::Instr::Bltu { .. }
                        | crate::decode::Instr::Bgeu { .. }
                )
            {
                crate::interp::step(&mut cpu, &mut mem)?;
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
    #[test]
    fn interp_matches_tcg_interp() {
        let asm = "tests/bare_hello.S";
        let _ = std::process::Command::new("riscv64-linux-gnu-as")
            .args(["-march=rv64gc", asm, "-o", "/tmp/hello.o"])
            .status();
        let _ = std::process::Command::new("riscv64-linux-gnu-ld")
            .args(["-o", "/tmp/hello", "/tmp/hello.o"])
            .status();
        let hello = "/tmp/hello";
        let remu = "./target/debug/remu";
        let out1 = std::process::Command::new(remu)
            .args(["--mode", "interp", hello])
            .output()
            .expect("interp");
        let out2 = std::process::Command::new(remu)
            .args(["--mode", "tcg-interp", hello])
            .output()
            .expect("tcg");
        let out3 = std::process::Command::new(remu)
            .args(["--mode", "jit", hello])
            .output()
            .expect("jit");
        assert_eq!(out1.status, out2.status);
        assert_eq!(out1.stdout, out2.stdout);
        assert_eq!(out1.status, out3.status);
        assert_eq!(out1.stdout, out3.stdout);
    }

    #[test]
    fn c_ext_matches_all_modes() {
        let asm = "tests/bare_hello_c.S";
        let _ = std::process::Command::new("riscv64-linux-gnu-as")
            .args(["-march=rv64gc", asm, "-o", "/tmp/hello_c.o"])
            .status();
        let _ = std::process::Command::new("riscv64-linux-gnu-ld")
            .args(["-o", "/tmp/hello_c", "/tmp/hello_c.o"])
            .status();
        let hello = "/tmp/hello_c";
        let remu = "./target/debug/remu";
        let out1 = std::process::Command::new(remu)
            .args(["--mode", "interp", hello])
            .output()
            .expect("interp");
        let out2 = std::process::Command::new(remu)
            .args(["--mode", "tcg-interp", hello])
            .output()
            .expect("tcg");
        let out3 = std::process::Command::new(remu)
            .args(["--mode", "jit", hello])
            .output()
            .expect("jit");
        assert_eq!(out1.status, out2.status);
        assert_eq!(out1.stdout, out2.stdout);
        assert_eq!(out1.status, out3.status);
        assert_eq!(out1.stdout, out3.stdout);
    }

    #[test]
    fn mem_matches_all_modes() {
        let asm = "tests/bare_mem.S";
        let _ = ::std::process::Command::new("riscv64-linux-gnu-as")
            .args(["-march=rv64gc", asm, "-o", "/tmp/bare_mem.o"])
            .status();
        let _ = ::std::process::Command::new("riscv64-linux-gnu-ld")
            .args(["-o", "/tmp/bare_mem", "/tmp/bare_mem.o"])
            .status();
        let elf = "/tmp/bare_mem";
        let remu = "./target/debug/remu";
        for &mode in &["interp", "tcg-interp", "jit"] {
            let out = ::std::process::Command::new(remu)
                .args(["--mode", mode, elf])
                .output()
                .expect(mode);
            assert_eq!(out.status.code(), Some(42), "{}", mode);
            assert_eq!(out.stdout, b"MEMTEST\n", "{}", mode);
        }
    }

    #[test]
    fn arith_and_branches_match_all_modes() {
        let asm = "tests/bare_arith.S";
        let _ = ::std::process::Command::new("riscv64-linux-gnu-as")
            .args(["-march=rv64gc", asm, "-o", "/tmp/bare_arith.o"])
            .status();
        let _ = ::std::process::Command::new("riscv64-linux-gnu-ld")
            .args(["-o", "/tmp/bare_arith", "/tmp/bare_arith.o"])
            .status();
        let elf = "/tmp/bare_arith";
        let remu = "./target/debug/remu";
        for &mode in &["interp", "tcg-interp", "jit"] {
            let out = ::std::process::Command::new(remu)
                .args(["--mode", mode, elf])
                .output()
                .expect(mode);
            assert_eq!(out.status.code(), Some(42), "{}", mode);
            assert_eq!(out.stdout, b"OK\n", "{}", mode);
        }
    }

    #[test]
    fn fp_matches_all_modes() {
        let asm = "tests/bare_fp.S";
        let _ = ::std::process::Command::new("riscv64-linux-gnu-as")
            .args(["-march=rv64gc", asm, "-o", "/tmp/bare_fp.o"])
            .status();
        let _ = ::std::process::Command::new("riscv64-linux-gnu-ld")
            .args(["-o", "/tmp/bare_fp", "/tmp/bare_fp.o"])
            .status();
        let elf = "/tmp/bare_fp";
        let remu = "./target/debug/remu";
        for &mode in &["interp", "tcg-interp", "jit"] {
            let out = ::std::process::Command::new(remu)
                .args(["--mode", mode, elf])
                .output()
                .expect(mode);
            assert_eq!(out.status.code(), Some(42), "{}", mode);
        }
    }
}
