mod cpu;
mod memory;
mod elf_loader;
mod syscall;
mod decode;
mod interp;
mod tcg;

#[derive(clap::Parser)]
#[command(version, about = "remu - RISC-V emulator with TCG")]
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
            crate::tcg::backend::execute_tcg(&ctx, &mut cpu, &mut mem);
            cpu.pc = end_pc;
            // If ended at unhandled control (branch), execute it with interp to update pc
            if let Ok((_, last)) = crate::decode::fetch_decode(&mem, cpu.pc)
                && matches!(last, crate::decode::Instr::Beq { .. } | crate::decode::Instr::Bne { .. } | crate::decode::Instr::Blt { .. } | crate::decode::Instr::Bge { .. } | crate::decode::Instr::Bltu { .. } | crate::decode::Instr::Bgeu { .. })
            {
                crate::interp::step(&mut cpu, &mut mem)?;
            }
        } else {
            anyhow::bail!("only interp and tcg-interp modes supported");
        }
        steps += 1;
        if steps > 10_000_000 {
            anyhow::bail!("step limit");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn interp_matches_tcg_interp() {
        let asm = "tests/bare_hello.S";
        let _ = Command::new("riscv64-linux-gnu-as").args(["-march=rv64gc", asm, "-o", "/tmp/hello.o"]).status();
        let _ = Command::new("riscv64-linux-gnu-ld").args(["-o", "/tmp/hello", "/tmp/hello.o"]).status();
        let hello = "/tmp/hello";
        let remu = "./target/debug/remu";
        let out1 = Command::new(remu).args(["--mode", "interp", hello]).output().expect("interp");
        let out2 = Command::new(remu).args(["--mode", "tcg-interp", hello]).output().expect("tcg");
        assert_eq!(out1.status, out2.status);
        assert_eq!(out1.stdout, out2.stdout);
    }
}
