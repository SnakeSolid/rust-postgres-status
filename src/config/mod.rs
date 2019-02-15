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
    servers: Vec<ServerConfig>,
    users: Vec<UserConfig>,
}

impl Config {
    pub fn update_interval(&self) -> u64 {
        self.update_interval
    }

    pub fn servers(&self) -> &[ServerConfig] {
        &self.servers
    }

    pub fn users(&self) -> &[UserConfig] {
        &self.users
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    disk_capacity: u64,
    soft_threshold: u64,
    hard_threshold: u64,
    host: String,
    port: u16,
    role: String,
    password: String,
    keep_database: HashSet<String>,
}

impl ServerConfig {
    pub fn disk_capacity(&self) -> u64 {
        self.disk_capacity
    }

    pub fn soft_threshold(&self) -> u64 {
        self.soft_threshold
    }

    pub fn hard_threshold(&self) -> u64 {
        self.hard_threshold
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

    pub fn keep_database(&self) -> &HashSet<String> {
        &self.keep_database
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserConfig {
    name: String,
    mail: Option<String>,
}

impl UserConfig {
    pub fn name(&self) -> &str {
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
