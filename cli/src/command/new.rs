use std::path::Path;
use std::process::Command;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::command::util::write_file;
use crate::ExitOk;
use model::Package;
use crate::file_backed_struct::FileBackedStruct;

pub enum Template {
    Lib,
    Bin,
}

static NEW_SCRIPT: &str = r#"
mkdir -p "$NAME";
cd "$NAME";
npm install -D typescript
./node_modules/.bin/tsc --init
mkdir src
"#;

static JUSTFILE: &str = r#"
set dotenv-load := true

export PATH := "./node_modules/.bin:" + env_var('PATH')

help:
    @just --list --unsorted

# Install all packages, build development assets. Make the project ready to run in development.
bootstrap:
    pnpm install

check:
    tsc --noEmit
alias c := check
"#;

static LIB_TS: &str = r#"
const world = "world";

export function hello(who: string = world): string {
    return `Hello ${who}!`;
}
"#;

static MAIN_TS: &str = r#"
function main() {
    console.log("Hello, World!")
}

main()
"#;

pub fn new(name: String, template: Template) -> Result<()> {
    Command::new("bash")
        .env("NAME", &name)
        .arg("-c")
        .arg(NEW_SCRIPT)
        .status()?
        .exit_ok()?;

    let path = Path::new(&name);
    match template {
        Template::Lib => {
            write_file(path.join("src/lib.ts"), LIB_TS.trim())?;
        }
        Template::Bin => {
            write_file(path.join("src/main.ts"), MAIN_TS.trim())?;
        }
    }
    write_file(path.join("Justfile"), JUSTFILE)?;

    let mut package = FileBackedStruct::<Package>::open(path.join("package.json"))?;
    package.name = Some(name);
    Ok(())
}