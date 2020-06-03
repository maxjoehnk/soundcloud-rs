# soundcloud
[![Build Status](https://travis-ci.org/maxjoehnk/soundcloud-rs.svg?branch=master)](https://travis-ci.org/maxjoehnk/soundcloud-rs)
[![Docs](https://docs.rs/soundcloud/badge.svg)](https://docs.rs/soundcloud)

A Rust library for interacting with the SoundCloud HTTP API.

## Usage

Add the following to your Cargo.toml file.

```toml
[dependencies]
soundcloud = "0.4"
```

To use this crate you need a client id.
Soundcloud currently doesn't allow signup for their api so you need to use an existing client id.

```rust
use soundcloud::Client;

#[tokio::main]
async fn main() {
    let client_id = std::env::var("SOUNDCLOUD_CLIENT_ID").unwrap();
    let client = Client::new(&client_id);
    // ...
}
```

API Usage is documented on [docs.rs](https://docs.rs/soundcloud).
