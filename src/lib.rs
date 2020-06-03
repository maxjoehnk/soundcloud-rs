// Copyright 2016 Mikkel Kroman <mk@uplink.io>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
//! SoundCloud API library
//!
//! This soundcloud library provides an interface where you can query soundcloud for information
//! about tracks and users.

pub use crate::app::App;
pub use crate::client::Client;
pub use crate::comment::Comment;
pub use crate::error::Error;
pub use crate::page::Page;
pub use crate::playlist::Playlist;
// Re-export commonly used resources.
pub use crate::track::Track;
pub use crate::user::User;

/// The static host address for the API.
pub const API_HOST: &str = "api.soundcloud.com";

mod app;
mod client;
mod comment;
pub mod error;
mod page;
mod playlist;
mod track;
mod user;
