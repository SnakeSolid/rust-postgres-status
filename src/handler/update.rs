use super::util::handle_empty;
use super::HandlerError;
use crate::config::ConfigRef;
use crate::state::StateRef;
use crate::worker;
use iron::middleware::Handler;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;

#[derive(Debug)]
pub struct UpdateHandler {
    config: ConfigRef,
    state: StateRef,
}

impl UpdateHandler {
    pub fn new(config: ConfigRef, state: StateRef) -> UpdateHandler {
        UpdateHandler { config, state }
    }
}

impl Handler for UpdateHandler {
    fn handle(&self, _request: &mut IronRequest) -> IronResult<IronResponse> {
        handle_empty(move || {
            worker::update_disk(&self.config, &self.state)
                .map_err(|err| HandlerError::new(&format!("{}", err)))?;
            worker::update_databases(&self.config, &self.state)
                .map_err(|err| HandlerError::new(&format!("{}", err)))
        })
    }
}
