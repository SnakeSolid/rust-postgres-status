use crate::state::StateError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type WorkerResult<T> = Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    StateError { message: String },
}

impl WorkerError {
    #[allow(clippy::needless_pass_by_value)]
    pub fn state_error(error: StateError) -> WorkerError {
        WorkerError::StateError {
            message: format!("{}", error),
        }
    }
}

impl Error for WorkerError {}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            WorkerError::StateError { message } => write!(f, "{}", message),
        }
    }
}
