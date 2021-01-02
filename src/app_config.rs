use std::str::FromStr;

pub use config::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub mappings: Vec<MappingDefinition>,
}


#[derive(Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MappingDefinition {
    pub name: String,
    pub path: String,
    pub flags: MappingFlags,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct MappingFlags {
    pub update: bool,
    pub delete: bool,
    pub create: bool,
}

impl FromStr for MappingFlags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            update: s.contains('u'),
            delete: s.contains('d'),
            create: s.contains('c'),
        })
    }
}

impl Default for MappingFlags {
    fn default() -> Self {
        return MappingFlags { update: false, delete: false, create: false };
    }
}

impl MappingDefinition {

}

impl FromStr for MappingDefinition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(";");
        let name = parts.next().unwrap();
        let path = parts.next().unwrap();
        let mut flags = match parts.next() {
            Some(fl) => MappingFlags::from_str(fl)?,
            None => MappingFlags::default(),
        };

        Ok(Self {
            name: name.into(),
            path: path.into(),
            flags,
        })
    }
}



impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub cfg: AppConfig,
}