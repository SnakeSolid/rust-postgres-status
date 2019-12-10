use crate::config::ConfigRef;
use crate::config::Cors;
use crate::handler::DropDbHandler;
use crate::handler::StateHandler;
use crate::handler::UpdateHandler;
use crate::options::Options;
use crate::state::StateRef;
use iron::Chain;
use iron::Iron;
use iron_cors::CorsMiddleware;
use mount::Mount;
use staticfile::Static;

#[allow(clippy::needless_pass_by_value)]
pub fn start(options: &Options, config: ConfigRef, state: StateRef) {
    let mut mount = Mount::new();
    mount.mount(
        "/api/v1/state",
        StateHandler::new(config.clone(), state.clone()),
    );
    mount.mount(
        "/api/v1/update",
        UpdateHandler::new(config.clone(), state.clone()),
    );
    mount.mount("/api/v1/dropdb", DropDbHandler::new(config.clone()));
    mount.mount("/static", Static::new("public/static"));
    mount.mount("/", Static::new("public"));

    let chain = make_chain(config, mount);
    let address = options.address();
    let port = options.port();

    println!("Listening on {}:{}...", address, port);

    match Iron::new(chain).http((address, port)) {
        Ok(_) => {}
        Err(err) => error!("Failed to start HTTP server: {}", err),
    }
}

fn make_chain(config: ConfigRef, mount: Mount) -> Chain {
    let mut chain = Chain::new(mount);

    match config.cors() {
        Some(Cors::AllowAny) => {
            chain.link_around(CorsMiddleware::with_allow_any());
        }
        Some(Cors::Whitelist { ref whitelist }) => {
            chain.link_around(CorsMiddleware::with_whitelist(whitelist.clone()));
        }
        None => {}
    }

    chain
}
