use std::fs::File;
use std::io::Write;
use std::path::Path;
use anyhow::Result;

pub fn write_file(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    let mut file = File::create(path.as_ref())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}