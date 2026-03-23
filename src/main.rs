use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
use commands::init;
use commands::hash_object;
mod repo;

#[derive(Parser)]
#[command(name = "vip")]
#[command(about = "A CLI tool for version control")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
    HashObject {
        path: PathBuf,
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => {
            init()?;
        }
        Command::HashObject { path } => {
            hash_object(&path)?;
        }

    }

    Ok(())
}