use leveldb::database::Database;
use leveldb::error::Error;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions};
use tempdir::TempDir;

pub struct DataStorage {
    name: String,
    path: String,
    database: Database<i32>,
}

impl DataStorage {
    pub fn new(name: String, path: String) -> Self {
        let tempdir = TempDir::new(path.as_str()).unwrap();
        let db_path = tempdir.path();
        let mut options = Options::new();
        options.create_if_missing = true;
        let mut database = Database::open(db_path, options).unwrap();
        DataStorage {
            name,
            path,
            database,
        }
    }

    pub fn save(&self, options: WriteOptions, key: i32, bytes: &[u8]) -> Result<(), Error> {
        self.database.put(options, key, bytes)
    }
}
