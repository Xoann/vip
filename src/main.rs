use clap::{Parser, Subcommand};

mod commands;
use commands::init;

#[derive(Parser)]
#[command(name = "vip")]
#[command(about = "A CLI tool for version control")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => {
            init()?;
        }
    }

    Ok(())
}