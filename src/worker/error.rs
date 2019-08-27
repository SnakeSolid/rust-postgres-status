use crate::state::StateError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;

pub type WorkerResult<T> = Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    StateError { message: String },
    IoError { message: String },
}

impl WorkerError {
    #[allow(clippy::needless_pass_by_value)]
    pub fn state_error(error: StateError) -> WorkerError {
        WorkerError::StateError {
            message: format!("{}", error),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn io_error(error: IoError) -> WorkerError {
        WorkerError::IoError {
            message: format!("{}", error),
        }
    }
}

impl Error for WorkerError {}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            WorkerError::StateError { message } => write!(f, "{}", message),
            WorkerError::IoError { message } => write!(f, "{}", message),
        }
    }
}
