use super::util::handle_request;
use super::HandlerError;
use crate::config::ConfigRef;
use crate::postgres::PostgreSQL;
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
            let name = request.name;
            let server_config = self.config.server();
            let postgres = PostgreSQL::new(
                server_config.host(),
                server_config.port(),
                server_config.role(),
                server_config.password(),
            );

            postgres.drop_database(&name).map_err(|err| {
                HandlerError::new(&format!("Failed to drop database `{}` - {}", name, err))
            })
        })
    }
}

#[derive(Debug, Deserialize)]
struct Request {
    name: String,
}
