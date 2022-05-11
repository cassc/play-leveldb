# A demo of using leveldb in Rust

This demo shows how to use customized data types as keys in leveldb
with Rust. All you need to do is to implment the
`leveldb::database::key::Key` trait for your data type which is to
used as leveldb keys.


For example, to use `MyData` as keys:

``` rust
use leveldb::database::key::Key;

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
```
