//! SoundCloud API library
//!
//! This soundcloud library provides an interface where you can query soundcloud for information
//! about tracks and users.

pub use crate::models::App;
pub use crate::client::Client;
pub use crate::error::{Error, Result};
pub use crate::apis::*;
pub use crate::models::*;
pub use crate::streaming_api::StreamingApiExt;
pub use crate::page::PageOptions;

/// The static host address for the API.
pub const API_HOST: &str = "https://api.soundcloud.com";

mod client;
pub mod error;
mod page;
mod streaming_api;
mod models;
mod apis;
