use super::util::handle_empty;
use super::HandlerError;
use crate::config::ConfigRef;
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
            let mut databases: Vec<DatabaseData> = Vec::new();

            self.state
                .for_each(|database| {
                    databases.push(DatabaseData::new(
                        database.name(),
                        database.modified(),
                        database.size(),
                    ))
                })
                .map_err(|_| HandlerError::new("State error"))?;

            let disk = self.config.server().disk();

            Ok(Response::new(
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
    disk_capacity: u64,
    soft_threshold: u64,
    hard_threshold: u64,
    databases: Vec<DatabaseData>,
}

impl Response {
    fn new(
        disk_capacity: u64,
        soft_threshold: u64,
        hard_threshold: u64,
        databases: Vec<DatabaseData>,
    ) -> Response {
        Response {
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
    modified: i64,
    size: u64,
}

impl DatabaseData {
    fn new(name: &str, modified: i64, size: u64) -> DatabaseData {
        DatabaseData {
            name: name.into(),
            modified,
            size,
        }
    }
}
