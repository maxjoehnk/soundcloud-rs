//! SoundCloud API library
//!
//! This soundcloud library provides an interface where you can query soundcloud for information
//! about tracks and users.

pub use crate::apis::*;
pub use crate::client::Client;
pub use crate::error::{Error, Result};
pub use crate::models::App;
pub use crate::models::*;
pub use crate::page::PageOptions;
pub use crate::streaming_api::StreamingApiExt;

/// The static host address for the API.
pub const API_HOST: &str = "https://api-v2.soundcloud.com";

mod apis;
mod client;
pub mod error;
mod models;
mod page;
mod streaming_api;
