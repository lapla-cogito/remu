mod cpu;
mod memory;
mod elf_loader;
mod syscall;
mod decode;

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
    println!("loaded entry {:#x} sp {:#x}", cpu.pc, cpu.read_gpr(2));
    if args.trace {
        println!("trace enabled (not yet)");
    }
    println!("mode: {}", args.mode);
    Ok(())
}
