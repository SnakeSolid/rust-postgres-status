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
            let server = &self
                .config
                .servers()
                .get(0)
                .ok_or_else(|| HandlerError::new("Server not found"))?;
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

            Ok(Response::new(
                server.disk_capacity(),
                server.soft_threshold(),
                server.hard_threshold(),
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
