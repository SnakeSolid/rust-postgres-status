use super::util::handle_request;
use super::HandlerError;
use crate::config::ConfigRef;
use crate::worker;
use iron::middleware::Handler;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;

#[derive(Debug)]
pub struct DropDbHandler {
    config: ConfigRef,
}

impl DropDbHandler {
    pub fn new(config: ConfigRef) -> DropDbHandler {
        DropDbHandler { config }
    }
}

impl Handler for DropDbHandler {
    fn handle(&self, request: &mut IronRequest) -> IronResult<IronResponse> {
        handle_request(request, move |request: Request| {
            worker::drop_database(&self.config, &request.name)
                .map_err(|err| HandlerError::new(&format!("{}", err)))
        })
    }
}

#[derive(Debug, Deserialize)]
struct Request {
    name: String,
}
