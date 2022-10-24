mod command;
mod exit_status;
mod file_backed_struct;

pub use exit_status::ExitOk;
pub use file_backed_struct::FileBackedStruct;

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, ValueEnum)]
enum SemverLevel {
    Major,
    Minor,
    Patch,
}

#[derive(Subcommand)]
enum Command {
    New {
        name: String,

        #[clap(long)]
        lib: bool,
    },
    SetVersion {
        #[clap(long)]
        bump: SemverLevel,
    },
    Publish {

    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New { name, lib } => {
            command::new(name, if lib { command::Template::Lib } else { command::Template::Bin })
        }
        Command::SetVersion { bump } => {
            unimplemented!()
        }
        Command::Publish {} => {
            unimplemented!()
        }
    }
}
