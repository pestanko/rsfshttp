use std::path::{Path, PathBuf};

pub struct PathWrapper {
    base: PathBuf,
    relative: PathBuf,
}

impl PathWrapper {
    pub fn new(base: &Path, relative: &Path) -> Self {
        Self {
            base: PathBuf::from(base),
            relative: PathBuf::from(relative),
        }
    }

    pub fn path(&self) -> &Path {
        &self.relative
    }

    pub fn base_path(&self) -> &Path {
        &self.base
    }

    pub fn full_path(&self) -> PathBuf {
        self.base.join(&self.relative)
    }
}
