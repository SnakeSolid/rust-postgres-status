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

    pub fn disk_state(&self) -> StateResult<DiskState> {
        self.with_read(move |state| Ok(state.disk_state()))
    }

    pub fn set_disk_state(
        &self,
        offset: u64,
        capacity: u64,
        soft_threshold: u64,
        hard_threshold: u64,
    ) -> StateResult<()> {
        self.with_write(move |state| {
            state.set_disk_state(offset, capacity, soft_threshold, hard_threshold);

            Ok(())
        })
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

    pub fn put(
        &self,
        name: &str,
        user: Option<&String>,
        modified: i64,
        size: u64,
    ) -> StateResult<()> {
        self.with_write(move |state| Ok(state.put(name, user, modified, size)))
    }

    pub fn clear(&self) -> StateResult<()> {
        self.with_write(move |state| Ok(state.clear()))
    }
}

#[derive(Debug, Clone)]
pub struct DiskState {
    offset: u64,
    capacity: u64,
    soft_threshold: u64,
    hard_threshold: u64,
}

impl DiskState {
    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn soft_threshold(&self) -> u64 {
        self.soft_threshold
    }

    pub fn hard_threshold(&self) -> u64 {
        self.hard_threshold
    }
}

impl Default for DiskState {
    fn default() -> Self {
        DiskState {
            offset: 0,
            capacity: 0,
            soft_threshold: 0,
            hard_threshold: 0,
        }
    }
}

#[derive(Debug)]
struct State {
    databases: HashMap<String, Database>,
    disk_state: DiskState,
}

impl State {
    #[allow(clippy::needless_pass_by_value)]
    fn new() -> State {
        State {
            databases: HashMap::new(),
            disk_state: DiskState::default(),
        }
    }

    pub fn disk_state(&self) -> DiskState {
        self.disk_state.clone()
    }

    pub fn set_disk_state(
        &mut self,
        offset: u64,
        capacity: u64,
        soft_threshold: u64,
        hard_threshold: u64,
    ) {
        self.disk_state.offset = offset;
        self.disk_state.capacity = capacity;
        self.disk_state.soft_threshold = soft_threshold;
        self.disk_state.hard_threshold = hard_threshold;
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Database),
    {
        for database in self.databases.values() {
            callback(database);
        }
    }

    fn put(&mut self, name: &str, user: Option<&String>, modified: i64, size: u64) {
        let database = Database::new(name, user, modified, size);

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
