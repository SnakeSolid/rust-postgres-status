mod postgres;

pub use self::postgres::DatabaseError;
pub use self::postgres::PostgreSQL;

use crate::config::ConfigRef;
use crate::config::ServerConfig;
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

            for server in self.config.servers() {
                self.query_databases(server);
            }

            info!("Query complete");

            thread::sleep(Duration::from_secs(interval));
        }
    }

    fn query_databases(&self, server: &ServerConfig) {
        let postgres = PostgreSQL::new(
            server.host(),
            server.port(),
            server.role(),
            server.password(),
        );

        match postgres.database_list(|name, modified, size| DatabaseInfo::new(name, modified, size))
        {
            Ok(infos) => {
                self.state.clear().unwrap();

                for info in infos {
                    self.state
                        .put(&info.name, info.modified, info.size)
                        .unwrap();
                }
            }
            Err(err) => warn!("Failed to retain old paths - {}", err),
        }
    }
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
