#[derive(Debug)]
pub struct Database {
    name: String,
    modified: i64,
    size: u64,
}

impl Database {
    pub fn new(name: &str, modified: i64, size: u64) -> Database {
        Database {
            name: name.into(),
            modified,
            size,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn modified(&self) -> i64 {
        self.modified
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
