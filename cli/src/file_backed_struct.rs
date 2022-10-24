use serde::Serialize;
use std::path::Path;

pub struct FileBackedStruct<T: serde::Serialize> {
    path: std::path::PathBuf,
    inner: T,
    pretty: bool,
}

impl<T> FileBackedStruct<T> where T: Serialize + for<'de> serde::Deserialize<'de> {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let inner = serde_json::from_reader(std::fs::File::open(&path)?)?;
        Ok(Self {
            path,
            inner,
            pretty: true,
        })
    }
}

impl<T: serde::Serialize> Drop for FileBackedStruct<T> {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(&self.path).unwrap();
        if self.pretty {
            serde_json::to_writer_pretty(&mut file, &self.inner).unwrap();
        } else {
            serde_json::to_writer(&mut file, &self.inner).unwrap();
        }
        eprintln!("{}: Wrote file.", self.path.display());
    }
}

impl<T: serde::Serialize> std::ops::Deref for FileBackedStruct<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: serde::Serialize> std::ops::DerefMut for FileBackedStruct<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
