use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type StateResult<T> = Result<T, StateError>;

#[derive(Debug)]
pub struct StateError {
    message: String,
}

impl StateError {
    pub fn new(message: &str) -> StateError {
        StateError {
            message: message.into(),
        }
    }
}

impl Error for StateError {}

impl Display for StateError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}
