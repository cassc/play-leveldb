extern crate leveldb;

use eyre::Result;
use leveldb::database::key::Key;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use std::path::Path;
use std::str;

#[derive(Clone)]
struct MyData {
    key: String,
}

impl Key for MyData {
    fn from_u8(key: &[u8]) -> Self {
        MyData {
            key: str::from_utf8(key).unwrap().into(),
        }
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        let dst = self.key.as_bytes();
        f(&dst)
    }
}

pub fn get_db<T: Key>(db_path: &str) -> Result<Database<T>> {
    let mut options = Options::new();
    options.create_if_missing = true;
    let db: Database<T> = Database::open(Path::new(db_path), options)?;
    Ok(db)
}

fn main() {
    let database = get_db(".database").unwrap();

    let write_opts = WriteOptions::new();
    let key = MyData {
        key: "hello".into(),
    };
    let value: String = "world".into();
    match database.put(write_opts, key.clone(), value.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            panic!("failed to write to database: {:?}", e)
        }
    };

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, key).unwrap().unwrap();
    let data = str::from_utf8(&res).unwrap();

    assert_eq!(data, value);
}
