use leveldb::database::Database;
use leveldb::error::Error;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;
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
        let database = Database::open(db_path, options).unwrap();
        DataStorage {
            name,
            path,
            database,
        }
    }

    pub fn save<T>(&self, options: WriteOptions, key: i32, data: &T) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let bytes = serde_json::to_string(data).unwrap();
        self.database.put(options, key, bytes.as_bytes())
    }

    pub fn read<T>(&self, options: ReadOptions<i32>, key: i32) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.database.get(options, key);
        return match response {
            Ok(data) => {
                let data_vec = data.unwrap();
                Result::Ok(serde_json::from_slice(data_vec.as_slice()).unwrap())
            }
            Err(e) => Result::Err(e),
        };
    }

    pub fn delete(&self, options: WriteOptions, key: i32) -> Result<(), Error> {
        self.database.delete(options, key)
    }
}
