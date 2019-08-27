use super::util::handle_empty;
use super::HandlerError;
use crate::config::ConfigRef;
use crate::config::DiskConfig;
use crate::state::StateRef;
use iron::middleware::Handler;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;

#[derive(Debug)]
pub struct StateHandler {
    config: ConfigRef,
    state: StateRef,
}

impl StateHandler {
    pub fn new(config: ConfigRef, state: StateRef) -> StateHandler {
        StateHandler { config, state }
    }
}

impl Handler for StateHandler {
    fn handle(&self, _request: &mut IronRequest) -> IronResult<IronResponse> {
        handle_empty(move || {
            let service_databases = self.config.server().service_databases();
            let mut databases: Vec<DatabaseData> = Vec::new();

            self.state
                .for_each(|database| {
                    databases.push(DatabaseData::new(
                        database.name(),
                        database.user(),
                        database.modified(),
                        database.size(),
                        service_databases.contains(database.name()),
                    ))
                })
                .map_err(|_| HandlerError::new("State error"))?;

            let disk = self
                .state
                .disk_state()
                .map_err(|_| HandlerError::new("State error"))?;
            let used: u64 = match self.config.server().disk() {
                DiskConfig::Fixed { .. } => databases.iter().map(|d| d.size).sum(),
                DiskConfig::Command { .. } => 0,
            };

            Ok(Response::new(
                disk.offset() + used,
                disk.capacity(),
                disk.soft_threshold(),
                disk.hard_threshold(),
                databases,
            ))
        })
    }
}

#[derive(Debug, Serialize)]
struct Response {
    disk_used: u64,
    disk_capacity: u64,
    soft_threshold: u64,
    hard_threshold: u64,
    databases: Vec<DatabaseData>,
}

impl Response {
    fn new(
        disk_used: u64,
        disk_capacity: u64,
        soft_threshold: u64,
        hard_threshold: u64,
        databases: Vec<DatabaseData>,
    ) -> Response {
        Response {
            disk_used,
            disk_capacity,
            soft_threshold,
            hard_threshold,
            databases,
        }
    }
}

#[derive(Debug, Serialize)]
struct DatabaseData {
    name: String,
    user: Option<String>,
    modified: i64,
    size: u64,
    service: bool,
}

impl DatabaseData {
    fn new(
        name: &str,
        user: Option<&String>,
        modified: i64,
        size: u64,
        service: bool,
    ) -> DatabaseData {
        DatabaseData {
            name: name.into(),
            user: user.cloned(),
            modified,
            size,
            service,
        }
    }
}
