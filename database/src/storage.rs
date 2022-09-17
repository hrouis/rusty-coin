macro_rules! data_storage {
    ($newtype:ident) => {
        pub struct $newtype {
            db: leveldb::database::Database<i32>,
        }

        impl $newtype {
            pub fn from_path(path: &std::path::Path) -> $newtype {
                let tempdir = tempdir::TempDir::new(path.to_str().unwrap()).unwrap();
                let db_path = tempdir.path();
                let mut options = leveldb::options::Options::new();
                options.create_if_missing = true;
                let database = leveldb::database::Database::<i32>::open(db_path, options).unwrap();
                $newtype { db: database }
            }

            pub fn save<T>(
                &self,
                options: leveldb::options::WriteOptions,
                key: i32,
                data: &T,
            ) -> Result<(), leveldb::error::Error>
            where
                T: serde::Serialize + serde::de::DeserializeOwned,
            {
                let bytes = serde_json::to_string(data).unwrap();
                self.db.put::<i32>(options, key, bytes.as_bytes())
            }

            pub fn read<T>(
                &self,
                options: leveldb::options::ReadOptions<i32>,
                key: i32,
            ) -> Result<T, leveldb::error::Error>
            where
                T: serde::Serialize + serde::de::DeserializeOwned,
            {
                let response = self.db.get(options, &key);
                return match response {
                    Ok(data) => {
                        let data_vec = data.unwrap();
                        Result::Ok(serde_json::from_slice(data_vec.as_slice()).unwrap())
                    }
                    Err(e) => Result::Err(e),
                };
            }

            pub fn delete(
                &self,
                options: leveldb::options::WriteOptions,
                key: i32,
            ) -> Result<(), leveldb::error::Error> {
                self.db.delete(options, key)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use leveldb::kv::KV;
    use leveldb::options::WriteOptions;

    data_storage!(BlockData);

    #[test]
    fn add_transaction_to_pool() {
        let path = std::string::String::from("./blockdata.dat");
        let block_storage = BlockData::from_path(Path::new(&path));
        match block_storage.save(WriteOptions::new(), 0, &std::string::String::from("Block")) {
            Ok(_) => {
                println!("Ok!")
            }
            Err(_) => {
                println!("error")
            }
        }
    }
}
