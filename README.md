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
pushover = "0.4.0"
```

Synchronous example:

```rust,no_run

use pushover::API;
use pushover::requests::message::SendMessage;

fn send_message() {
    let api = API::new();

    let msg = SendMessage::new("token", "user_key", "hello");

    let response = api.send(&msg);
    println!("{:?}", response.expect("Error sending message"));
}
```

Asynchronous example:

```rust,no_run

use pushover::API;
use pushover::requests::message::SendMessage;

async fn send_message() {
    let api = API::new();

    let msg = SendMessage::new("token", "user_key", "hello");
    let response = api.send_async(&msg).await;

    println!("{:?}", response.expect("Error sending message"));
}
```
