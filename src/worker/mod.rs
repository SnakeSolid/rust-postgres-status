mod error;
mod postgres;

pub use self::error::WorkerError;
pub use self::error::WorkerResult;
pub use self::postgres::DatabaseError;
pub use self::postgres::PostgreSQL;

use crate::config::ConfigRef;
use crate::state::StateRef;
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

        loop {
            info!("Start query databases");

            if let Err(err) = update_databases(&self.config, &self.state) {
                warn!("Update database error: {}", err);
            }

            info!("Query complete");

            thread::sleep(Duration::from_secs(interval));
        }
    }
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

/// Update state to match all databases in query. This function will block callee until all databases updated.
///
/// If some error occurred content of state is not defined.
pub fn drop_database(config: &ConfigRef, name: &str) -> WorkerResult<()> {
    let server_config = config.server();
    let postgres = PostgreSQL::new(
        server_config.host(),
        server_config.port(),
        server_config.role(),
        server_config.password(),
    );

    if let Err(err) = postgres.drop_database(name) {
        warn!("Failed to drop database - {}", err);
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
