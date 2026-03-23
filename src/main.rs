use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
use commands::init;
use commands::hash_object;
use commands::cat_file;
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
    },
    CatFile {
        #[arg(short = 'c')]
        show_content: bool,

        #[arg(short = 't')]
        show_type: bool,

        #[arg(short = 's')]
        show_size: bool, 

        hash: String,
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
         Command::CatFile {
            show_content,
            show_type,
            show_size,
            hash,
        } => {
            let mode = if show_content {
                commands::cat_file::CatFileMode::Content
            } else if show_type {
                commands::cat_file::CatFileMode::Type
            } else if show_size {
                commands::cat_file::CatFileMode::Size
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "must specify one of -p, -t, or -s",
                ));
            };

            cat_file(&hash, mode)?;
        }
    }

    Ok(())
}