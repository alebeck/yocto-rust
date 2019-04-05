//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use yocto::args::Config;
use yocto_client::error::*;
use log::LogLevelFilter;
use std::thread;
use std::time::Duration;

#[test]
fn unknown_key() {
    bootstrap(1);

    let store = yocto_rust::Store::new("127.0.0.1:7002");
    let result = store.get("key");
    if let Err(e) = result {
        return;
    }
    panic!();
}

#[test]
fn insert() {
    bootstrap(1);

    let store = yocto_rust::Store::new("127.0.0.1:7002");
    let result = store.insert("key", "value");
    if let Ok(inner) = result {
        if inner == None {
            return;
        }
    }
    panic!();
}

#[test]
fn existing_key() {
    bootstrap(2);

    let store = yocto_rust::Store::new("127.0.0.1:7002");
    let _ = store.insert("key", "value");
    let result = store.get("key");
    if let Ok(message) = result {
        assert_eq!(message, Some("value".to_string()));
    } else {
        panic!();
    }
}

#[test]
fn insert_existing() {
    bootstrap(2);

    let store = yocto_rust::Store::new("127.0.0.1:7002");
    let _ = store.insert("key", "value");
    let result = store.insert("key", "value2");
    if let Ok(inner) = result {
        if inner == Some("value".to_string()) {
            return;
        }
    }
    panic!();
}

#[test]
fn remove() {
    bootstrap(3);

    let store = yocto_rust::Store::new("127.0.0.1:7002");
    let _ = store.insert("key", "value");
    let result = store.remove("key");
    let result = store.get("key");
    if let Err(e) = result {} else {
        panic!();
    }
}

#[test]
fn clear() {
    bootstrap(2);

    let store = yocto_rust::Store::new("127.0.0.1:7002");
    let _ = store.insert("key", "value");
    let result = store.clear();
    if let Ok(None) = result {
        let result = store.get("key");
        if let Err(e) = result {
            return;
        }
    }
    panic!();
}

fn bootstrap(exit_after: usize) {
    let config = Config {
        threads: 1,
        iface: "127.0.0.1:7002".to_string(),
        log_level: LogLevelFilter::Error,
        exit_after: Some(exit_after)
    };

    let handle = thread::spawn(|| {
        yocto::run(config);
    });

    // give it some time to start
    thread::sleep(Duration::from_millis(200));
}