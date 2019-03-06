#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

mod config;
mod error;
mod handler;
mod options;
mod postgres;
mod server;
mod state;
mod worker;

use crate::error::ApplicationError;
use crate::error::ApplicationResult;
use crate::options::Options;
use structopt::StructOpt;

fn main() -> ApplicationResult {
    env_logger::init();

    let options = Options::from_args();
    let config =
        config::load(options.config_path()).map_err(ApplicationError::read_config_error)?;

    config::validate(config.clone()).map_err(ApplicationError::config_error)?;

    let state = state::create();

    worker::start(config.clone(), state.clone());
    server::start(&options, config, state);

    Ok(())
}
