use std::path::PathBuf;
use std::process::Command;

use clap::command;
use clap::Parser;
use clap::ValueHint;
use color_eyre::Report;

#[derive(Parser, Debug)]
#[command(author)]
struct Cli {
    #[clap(value_hint = ValueHint::FilePath)]
    kernel_bin: PathBuf,

    #[clap(value_hint = ValueHint::FilePath)]
    manifest_dir: PathBuf,

    #[clap(long, short, value_hint = ValueHint::FilePath)]
    uefi: Option<PathBuf>,

    #[clap(long, short, value_hint = ValueHint::FilePath, default_value = "qemu-system-x86_64")]
    qemu: PathBuf,
}

impl Cli {
    pub fn uefi(&self) -> PathBuf {
        self.uefi.clone().unwrap_or(ovmf_prebuilt::ovmf_pure_efi())
    }
}

fn run(cli: &Cli) -> Result<std::process::ExitStatus, Report> {
    let status = Command::new(&cli.qemu)
        .arg("-bios")
        .arg(cli.uefi())
        .arg("-kernel")
        .arg(&cli.kernel_bin)
        .arg("-no-reboot")
        .status()?;

    Ok(status)
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let status = run(&cli)?;

    println!("{status}");

    Ok(())
}
