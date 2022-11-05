use std::fs::File;
use std::path::PathBuf;
use model::Package;
use crate::FileBackedStruct;
use anyhow::{anyhow, Result};

pub struct Override {
    pub package: String,
    pub path: String,
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
                Ok(Override {
                    package: name.to_string(),
                    path: s,
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
    let mut pnpm = package.pnpm.get_or_insert(Default::default());
    let mut o = pnpm.overrides.get_or_insert(Default::default());

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
