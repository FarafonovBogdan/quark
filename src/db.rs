use sled::Db;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    db: Arc<Db>,
    read_only: bool,
}

impl Database {
    pub fn new(path: &str, read_only: bool) -> sled::Result<Self> {
        let db = sled::open(path)?;
        Ok(Self {
            db: Arc::new(db),
            read_only,
        })
    }

    pub fn set(&self, key: &str, value: &str) -> sled::Result<()> {
        println!("DB: Writing key={} value={}", key, value);
        if self.read_only {
            return Err(sled::Error::Unsupported("Read-only mode".into()));
        }
        self.db.insert(key, value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.db
            .get(key)
            .ok()
            .flatten()
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
    }

    pub fn delete(&self, key: &str) -> sled::Result<()> {
        if self.read_only {
            return Err(sled::Error::Unsupported("Read-only mode".into()));
        }
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }
}
