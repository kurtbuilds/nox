mod command;
mod exit_status;
mod file_backed_struct;

pub use exit_status::ExitOk;
pub use file_backed_struct::FileBackedStruct;

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use crate::command::strings_into_overrides;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, ValueEnum)]
pub enum SemverLevel {
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

    },
    Override {
        #[clap(long, action)]
        off: bool,
        overrides: Vec<String>
    }

}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New { name, lib } => {
            command::new(name, if lib { command::Template::Lib } else { command::Template::Bin })
        }
        Command::SetVersion { bump } => {
            command::bump(bump)
        }
        Command::Publish {} => {
            unimplemented!()
        }
        Command::Override { off , overrides } => {
            if off {
                command::clear_overrides("./package.json".into())
            } else {
                let overrides = strings_into_overrides(overrides)?;
                command::add_override(overrides, "./package.json".into())
            }
        }
    }
}