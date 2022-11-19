use std::process::ExitStatus;
use anyhow::{Result, anyhow};

pub trait ExitOk {
    fn exit(self) -> Result<()>;
}

impl ExitOk for ExitStatus {
    fn exit(self) -> Result<()> {
        if self.success() {
            Ok(())
        } else {
            Err(anyhow!("Process exited with status: {}", self))
        }
    }
}