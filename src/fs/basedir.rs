use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::{Path, PathBuf}, io, fmt::Display,
};

use glob;
use slog::{self, o};

pub struct Dirs {
    paths: HashMap<String, BasePath>,
    logger: slog::Logger,
}

impl Dirs {
    pub fn new(logger: &slog::Logger) -> Self {
        Self {
            paths: HashMap::new(),
            logger: logger.new(o!()),
        }
    }

    pub fn add(&mut self, name: &str, dir: BasePath) -> &Dirs {
        let name_str: String = String::from(name);
        self.paths.insert(String::from(name), dir);
        self
    }

    pub fn get(&self, name: &str) -> Option<&BasePath> {
        self.paths.get(name)
    }

    pub fn into_basepath(&self, path: &Path) -> BasePath {
       BasePath::new(path, &self.logger)
    }
}

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
        let new_path = self.path.join(path);
        if new_path.exists() {
            Some(DirectoryEntry::from(new_path))
        } else {
            slog::debug!(self.logger, "path not found"; "path" => path.to_str());
            None
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

#[derive(Clone, PartialEq)]
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