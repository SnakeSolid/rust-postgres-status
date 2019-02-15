mod database;
mod error;

pub use self::database::Database;
pub use self::error::StateError;
pub use self::error::StateResult;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct StateRef {
    inner: Arc<RwLock<State>>,
}

impl StateRef {
    fn with_read<F, T>(&self, callback: F) -> StateResult<T>
    where
        F: FnOnce(&State) -> StateResult<T>,
    {
        match self.inner.write() {
            Ok(ref state) => callback(state),
            Err(err) => {
                warn!("Failed to acquire write lock - {}", err);

                Err(StateError::new("Failed to acquire write lock"))
            }
        }
    }

    fn with_write<F, T>(&self, callback: F) -> StateResult<T>
    where
        F: FnOnce(&mut State) -> StateResult<T>,
    {
        match self.inner.write() {
            Ok(ref mut state) => callback(state),
            Err(err) => {
                warn!("Failed to acquire write lock - {}", err);

                Err(StateError::new("Failed to acquire write lock"))
            }
        }
    }

    pub fn for_each<F>(&self, callback: F) -> StateResult<()>
    where
        F: FnMut(&Database),
    {
        self.with_read(move |state| {
            state.for_each(callback);

            Ok(())
        })
    }

    pub fn put(&self, name: &str, modified: i64, size: u64) -> StateResult<()> {
        self.with_write(move |state| Ok(state.put(name, modified, size)))
    }

    pub fn clear(&self) -> StateResult<()> {
        self.with_write(move |state| Ok(state.clear()))
    }
}

#[derive(Debug)]
struct State {
    databases: HashMap<String, Database>,
}

impl State {
    #[allow(clippy::needless_pass_by_value)]
    fn new() -> State {
        State {
            databases: HashMap::new(),
        }
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Database),
    {
        for database in self.databases.values() {
            callback(database);
        }
    }

    fn put(&mut self, name: &str, modified: i64, size: u64) {
        let database = Database::new(name, modified, size);

        self.databases.insert(name.into(), database);
    }

    fn clear(&mut self) {
        self.databases.clear();
    }
}

pub fn create() -> StateRef {
    StateRef {
        inner: Arc::new(RwLock::new(State::new())),
    }
}
