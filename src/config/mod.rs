mod error;
mod validate;

pub use self::error::ConfigError;
pub use self::error::ConfigResult;
pub use self::validate::validate;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub type ConfigRef = Arc<Config>;

#[derive(Debug, Deserialize)]
pub struct Config {
    update_interval: u64,
    server: ServerConfig,
    users: Vec<UserConfig>,
}

impl Config {
    pub fn update_interval(&self) -> u64 {
        self.update_interval
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn users(&self) -> &[UserConfig] {
        &self.users
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    host: String,
    port: u16,
    role: String,
    password: String,
    service_databases: HashSet<String>,
    disk: DiskConfig,
}

impl ServerConfig {
    pub fn disk(&self) -> &DiskConfig {
        &self.disk
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn service_databases(&self) -> &HashSet<String> {
        &self.service_databases
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiskConfig {
    offset: u64,
    capacity: u64,
    soft_threshold: u64,
    hard_threshold: u64,
}

impl DiskConfig {
    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn soft_threshold(&self) -> u64 {
        self.soft_threshold
    }

    pub fn hard_threshold(&self) -> u64 {
        self.hard_threshold
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserConfig {
    login: String,
    name: Option<String>,
    mail: Option<String>,
}

impl UserConfig {
    pub fn login(&self) -> &str {
        self.login.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn mail(&self) -> Option<&String> {
        self.mail.as_ref()
    }
}

pub fn load<P>(path: P) -> ConfigResult<ConfigRef>
where
    P: AsRef<Path>,
{
    let reader = File::open(path).map_err(ConfigError::io_error)?;
    let config = serde_yaml::from_reader(reader).map_err(ConfigError::yaml_error)?;

    Ok(Arc::new(config))
}
