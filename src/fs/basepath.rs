use std::{
    fs::{self},
    path::{Path, PathBuf}, fmt::Display,
};

use glob;
use slog::{self, o};

use pathdiff::diff_paths;



pub struct BasePath {
    path: PathBuf,
    logger: slog::Logger,
}

impl BasePath {
    pub fn new(path: &Path, logger: &slog::Logger) -> Self {
        Self {
            path: PathBuf::from(path),
            logger: logger.new(o!("base_path" => path.to_str().unwrap().to_string())),
        }
    }

    pub fn get(&self, path: &Path) -> Option<DirectoryEntry> {
        match self.path.join(path).canonicalize() {
            Ok(new_path) => {
                if !is_forward_path(&new_path, &self.path) {
                    slog::warn!(self.logger, "path is not forward found"; "path" => new_path.display(), "base" => self.path.display());
                    None
                }
                else if new_path.exists() {
                    slog::debug!(self.logger, "canonized path found"; "path" => new_path.display());
                    Some(DirectoryEntry::from(new_path))
                } else {
                    slog::warn!(self.logger, "path not found"; "path" => path.display());
                    None
                }
            }
            Err(err) => {
                slog::debug!(self.logger, "path canonicalize failed"; "path" => path.to_str(), "error" => err.to_string());
                None
            }
        }
    }

    pub fn fglob(&self, pattern: &str) -> Vec<DirectoryEntry> {
        let fp = String::from(self.path.join(pattern).to_str().unwrap()); // this might fail
        match glob::glob(&fp) {
            Ok(paths) => { paths.into_iter().map(|x| match x {
                Ok(path) => DirectoryEntry::from(path),
                Err(err) => {
                    slog::error!(self.logger, "pattern error"; "err" => err.to_string(), "pattern" => pattern);
                    DirectoryEntry::Error(String::from("pattern error - unable to glob"))
                },
            }).collect() 
        },
        Err(err) => {
            slog::error!(self.logger, "unable to glob the directory";"err" => err.to_string(), "pattern" => pattern);
            Vec::new()
            }
        }
    }

    pub fn list(&self) -> Vec<DirectoryEntry> {
        match fs::read_dir(&self.path) {
            Ok(dir) => {
                dir.map(
                    |r| {
                        match r {
                            Ok(entry) => {
                                DirectoryEntry::from(entry.path())
                            }
                            Err(err) => {
                                slog::error!(self.logger, "Error reading entry"; "err" => err.to_string());
                                DirectoryEntry::Error(format!("list error - unable to read: {}", err.to_string()))
                            }
                        }
                    } 
                ).collect()
            },
            Err(err) => {
                slog::error!(self.logger, "unable to read the directory"; "err" => err.to_string());
                Vec::new()
            }
        }
    }
}


#[derive(Clone, PartialEq,Debug)]
pub enum DirectoryEntry {
    File(PathBuf),
    Dir(PathBuf),
    Unspecified(PathBuf), // Example: socket
    Error(String)
}

impl From<PathBuf> for DirectoryEntry {
    fn from(new_path: PathBuf) -> Self {
        if new_path.is_file() {
            Self::File(new_path).clone()
        } else if new_path.is_dir() {
            Self::Dir(new_path).clone()
        } else {
            Self::Unspecified(new_path.clone())
        }
    }
}

impl Display for DirectoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectoryEntry::File(fpath) => {
                write!(f, "{}", fpath.to_str().unwrap())
            }
            DirectoryEntry::Dir(dpath) => {
                write!(f, "{}", dpath.to_str().unwrap())
            }
            DirectoryEntry::Unspecified(pth) => {
                write!(f, "Unspecified({})", pth.to_str().unwrap())
            }
            DirectoryEntry::Error(err) => {
                write!(f, "Error: ({})", err)
            }
        }
    }
    
}

/**
 *
 * diff_paths(/etc/password, /tmp) ~> ../etc/passwd
 * diff_paths(/tmp/ahoj/svet, /tmp) ~> ahoj/svet
 */
fn is_forward_path(pth: &Path, base_pth: &Path) -> bool {
    if let Some(p) = diff_paths(pth, base_pth) {
        let s = format!("{}", p.display());
        return ! s.starts_with("../");
    }
    false
}