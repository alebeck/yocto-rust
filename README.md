# yocto-rust
 
[![Build Status](https://cloud.drone.io/api/badges/alebeck/yocto-rust/status.svg)](https://cloud.drone.io/alebeck/yocto-rust)
[![Crates.io](https://img.shields.io/crates/v/yocto_client.svg)](https://crates.io/crates/yocto_client)

Yocto client for rust.

## Usage

Include yocto_client in your projects `Cargo.toml` with the version specified at [crates.io](https://crates.io/crates/yocto_client). Then, use it as follows:

```rust
use yocto_client::Store;

let store = Store::new("127.0.0.1:7001").unwrap();

store.insert("key", "value").unwrap();

if let Some(value) = store.get("key").unwrap() {
    assert_eq!(value, "value");
} else {
    panic!("Key not found.");
}
```
