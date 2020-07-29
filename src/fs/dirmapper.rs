use std::{path::Path, collections::HashMap};
use super::BasePath;
use slog::{debug, o, warn};

pub struct DirsMapper {
    paths: HashMap<String, BasePath>,
    logger: slog::Logger,
}

impl DirsMapper {
    pub fn new(logger: &slog::Logger) -> Self {
        Self {
            paths: HashMap::new(),
            logger: logger.new(o!("type" => "DirMapper")),
        }
    }

    pub fn add(&mut self, name: &str, dir: BasePath) -> &DirsMapper {
        let name_str: String = String::from(name);
        self.paths.insert(String::from(name), dir);
        self
    }

    pub fn add_mapping(&mut self, name: &str, dir: &Path) -> &DirsMapper {
        let name_str: String = String::from(name);
        debug!(self.logger, "Adding mapping"; "name" => name, "path" => dir.display());
        match dir.canonicalize() {
            Ok(d) => {
                debug!(self.logger, "Adding canonized path"; "path" => d.display());
                self.paths.insert(String::from(name), self.into_basepath(dir));
            }
            Err(err) => {
                warn!(self.logger, "Unable to canonize"; "error" => err.to_string(), "path" => dir.display());
            }
        }
        self
    }

    pub fn get(&self, name: &str) -> Option<&BasePath> {
        self.paths.get(name)
    }

    fn into_basepath(&self, path: &Path) -> BasePath {
       BasePath::new(path, &self.logger)
    }

    pub fn paths(&self) -> &HashMap<String, BasePath> {
        &self.paths
    }
}