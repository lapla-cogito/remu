mod cpu;
mod memory;
mod elf_loader;
mod syscall;
mod decode;
mod interp;

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
    if args.mode != "interp" {
        anyhow::bail!("only interp mode supported");
    }
    let mut steps: u64 = 0;
    loop {
        crate::interp::step(&mut cpu, &mut mem)?;
        steps += 1;
        if steps > 10_000_000 {
            anyhow::bail!("step limit");
        }
    }
}
