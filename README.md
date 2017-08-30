Pushover
=========================
[![Build Status](https://img.shields.io/travis/sb89/pushover/master.svg)](https://travis-ci.org/sb89/pushover)
[![License](https://img.shields.io/github/license/sb89/pushover.svg)]()
[![Crates.io](https://img.shields.io/crates/v/pushover.svg)](https://crates.io/crates/pushover)
[![Docs.rs](https://docs.rs/pushover/badge.svg)](https://docs.rs/pushover)

A Rust wrapper for the Pushover API (https://pushover.net/api).

## Installation

## Usage
Add the following to `Cargo.toml`:

```toml
[dependencies]
pushover = "0.2.0"
```

Synchronous example:

```rust,no_run

extern crate pushover;

use pushover::SyncAPI;
use pushover::requests::message::SendMessage;

fn main() {
    let api = SyncAPI::new().expect("Error creating API");

    let msg = SendMessage::new("token", "user_key", "hello");

    let response = api.send(&msg);
    println!("{:?}", response);
}
```

Asynchronous example:

```rust,no_run

extern crate pushover;
extern crate tokio_core;

use pushover::{AsyncAPI};
use pushover::requests::message::SendMessage;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().expect("Error creating core");
    let handle = core.handle();

    let api = AsyncAPI::new(&handle).expect("Error creating API");

    let msg = SendMessage::new("token", "user_key", "hello");
    let work = api.send(&msg);

    println!("{:?}", core.run(work).expect("Error sending message"));
}
```
