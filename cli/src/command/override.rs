use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use model::{Package, TsConfig};
use crate::{ExitOk, FileBackedStruct};
use anyhow::{anyhow, Result};

pub struct Override {
    pub package: String,
    pub path: String,
}

fn remap_for_base_url(path: PathBuf) -> Result<PathBuf> {
    let config = serde_json::from_reader::<_, TsConfig>(File::open(path.join("tsconfig.json"))?)?;
    let comp = match config.compiler_options {
        None => return Ok(path),
        Some(c) => c
    };
    match comp.base_url {
        None => return Ok(path),
        Some(p) => Ok(path.join(p).components().filter(|c| !matches!(c, std::path::Component::CurDir)).collect::<PathBuf>())
        // Some(p) => Ok(path.join(p).canonicalize()?)
    }
}

pub fn strings_into_overrides(vec: Vec<String>) -> Result<Vec<Override>> {
    vec.into_iter().map(|s| {
        let count = s.chars().filter(|c| *c == '=').count();
        match count {
            0 => {
                let path = PathBuf::from(&s);
                if !path.exists() {
                    return Err(anyhow!("{}: Path does not exist.", s));
                }
                let package = serde_json::from_reader::<_, Package>(File::open(path.join("package.json"))?)?;
                let name = match &package.name {
                    Some(n) => n,
                    None => return Err(anyhow!("{} does not have a \"name\" key in package.json", s))
                };
                let path = remap_for_base_url(path)?.to_string_lossy().to_string();
                Ok(Override {
                    package: name.to_string(),
                    path,
                })
            }
            1 => {
                let mut split = s.splitn(2, "=");
                let package = split.next().expect("Each override must be structured as <package>=<path>").to_string();
                let path = split.next().expect("Each override must be structured as <package>=<path>").to_string();
                let path_buf = PathBuf::from(&path);
                if !path_buf.exists() {
                    return Err(anyhow!("{}: Path does not exist.", path));
                }
                if !path_buf.join("package.json").exists() {
                    return Err(anyhow!("{}: No package.json file found there.", path));
                }
                let path = remap_for_base_url(PathBuf::from(path))?.to_string_lossy().to_string();
                Ok(Override {
                    package,
                    path,
                })
            }
            _ => Err(anyhow!("Must provide a path or a <pkg>=<path>"))
        }
    }).collect::<Result<Vec<_>, _>>()
}


pub fn add_override(overrides: Vec<Override>, path: PathBuf) -> Result<()> {
    let mut package = FileBackedStruct::<Package>::open(path)?;
    let pnpm = package.pnpm.get_or_insert(Default::default());
    let o = pnpm.overrides.get_or_insert(Default::default());

    for over in overrides.into_iter() {
        o.insert(over.package, format!("link:{}", over.path));
    }
    Ok(())
}

pub fn clear_overrides(path: PathBuf) -> Result<()> {
    let mut package = FileBackedStruct::<Package>::open(path)?;
    let mut pnpm = package.pnpm.get_or_insert(Default::default());
    pnpm.overrides = None;
    if pnpm.is_empty() {
        package.pnpm = None;
    }
    Ok(())
}

pub fn run_override(off: bool, overrides: Vec<Override>, path: PathBuf) -> Result<()> {
    if off {
        clear_overrides(path)?;
    } else {
        add_override(overrides, path)?;
    }
    Command::new("pnpm")
        .arg("install")
        .status()?
        .exit()
}