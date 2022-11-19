use std::path::Path;
use model::Package;
use semver::Version;
use crate::{FileBackedStruct, SemverLevel};
use anyhow::Result;

pub fn bump(level: SemverLevel) -> Result<()> {
    let path = Path::new(".");
    let mut package = FileBackedStruct::<Package>::open(path.join("package.json"))?;
    let mut version = match &package.version {
        Some(version) => Version::parse(version)?,
        None => Version::new(0, 0, 0),
    };
    match level {
        SemverLevel::Major => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        }
        SemverLevel::Minor => {
            version.minor += 1;
            version.patch = 0;
        }
        SemverLevel::Patch => {
            version.patch += 1;
        }
    }
    package.version = Some(version.to_string());
    Ok(())
}