mod error;

pub use self::error::WorkerError;
pub use self::error::WorkerResult;

use crate::config::ConfigRef;
use crate::config::DiskConfig;
use crate::postgres::PostgreSQL;
use crate::state::StateRef;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::thread::Builder;
use std::time::Duration;

#[derive(Debug)]
pub struct Worker {
    config: ConfigRef,
    state: StateRef,
}

impl Worker {
    pub fn new(config: ConfigRef, state: StateRef) -> Worker {
        Worker { config, state }
    }

    fn start(self) {
        let interval = self.config.update_interval();

        if let DiskConfig::Fixed {
            offset,
            capacity,
            soft_threshold,
            hard_threshold,
        } = self.config.server().disk()
        {
            if let Err(err) =
                self.state
                    .set_disk_state(*offset, *capacity, *soft_threshold, *hard_threshold)
            {
                warn!("Update state error: {}", err);
            }
        }

        loop {
            info!("Start query databases");

            if let Err(err) = update_disk(&self.config, &self.state) {
                warn!("Update disk error: {}", err);
            }

            if let Err(err) = update_databases(&self.config, &self.state) {
                warn!("Update database error: {}", err);
            }

            info!("Query complete");

            thread::sleep(Duration::from_secs(interval));
        }
    }
}

/// Update disk usage info using command defined in the configuration. This function will block callee.
///
/// If some error occurred content of state is not defined.
pub fn update_disk(config: &ConfigRef, state: &StateRef) -> WorkerResult<()> {
    let mut offset = 0;
    let mut capacity = 0;
    let mut soft_threshold = 0;
    let mut hard_threshold = 0;

    if let DiskConfig::Command { command } = config.server().disk() {
        let mut child = Command::new(command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .env_clear()
            .spawn()
            .map_err(WorkerError::io_error)?;

        if let Some(ref mut stdout) = child.stdout {
            let mut buffer = String::default();

            stdout
                .read_to_string(&mut buffer)
                .map_err(WorkerError::io_error)?;

            let mut lines = buffer.lines().map(|line| line.trim());

            offset = lines
                .next()
                .map(|value| value.parse().unwrap_or(0))
                .unwrap_or(0);
            capacity = lines
                .next()
                .map(|value| value.parse().unwrap_or(0))
                .unwrap_or(0);
            soft_threshold = lines
                .next()
                .map(|value| value.parse().unwrap_or(0))
                .unwrap_or(0);
            hard_threshold = lines
                .next()
                .map(|value| value.parse().unwrap_or(0))
                .unwrap_or(0);
        }

        child.wait().map_err(WorkerError::io_error)?;
    }

    state
        .set_disk_state(offset, capacity, soft_threshold, hard_threshold)
        .map_err(WorkerError::state_error)
}

/// Update state to match all databases in query. This function will block callee until all databases updated.
///
/// If some error occurred content of state is not defined.
pub fn update_databases(config: &ConfigRef, state: &StateRef) -> WorkerResult<()> {
    let server_config = config.server();
    let all_users: Vec<_> = config
        .users()
        .iter()
        .map(|user| user.login().to_lowercase())
        .collect();
    let postgres = PostgreSQL::new(
        server_config.host(),
        server_config.port(),
        server_config.role(),
        server_config.password(),
    );

    match postgres.database_list(|name, modified, size| DatabaseInfo::new(name, modified, size)) {
        Ok(infos) => {
            state.clear().map_err(WorkerError::state_error)?;

            for info in infos {
                let database_name = info.name.to_lowercase();
                let user = all_users
                    .iter()
                    .find(|&login| database_name.contains(login));

                state
                    .put(&info.name, user, info.modified, info.size)
                    .map_err(WorkerError::state_error)?;
            }
        }
        Err(err) => warn!("Failed to retain old paths - {}", err),
    }

    Ok(())
}

pub fn start(config: ConfigRef, state: StateRef) {
    if let Err(err) = Builder::new()
        .name("state worker".to_string())
        .spawn(move || Worker::new(config, state).start())
    {
        warn!("Failed to start state worker - {}", err);
    }
}

#[derive(Debug)]
struct DatabaseInfo {
    name: String,
    modified: i64,
    size: u64,
}

impl DatabaseInfo {
    fn new(name: &str, modified: i64, size: u64) -> DatabaseInfo {
        DatabaseInfo {
            name: name.into(),
            modified,
            size,
        }
    }
}
