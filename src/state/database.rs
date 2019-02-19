#[derive(Debug)]
pub struct Database {
    name: String,
    user: Option<String>,
    modified: i64,
    size: u64,
}

impl Database {
    pub fn new(name: &str, user: Option<&String>, modified: i64, size: u64) -> Database {
        Database {
            name: name.into(),
            user: user.cloned(),
            modified,
            size,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn user(&self) -> Option<&String> {
        self.user.as_ref()
    }

    pub fn modified(&self) -> i64 {
        self.modified
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
