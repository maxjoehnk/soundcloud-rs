use serde::{Deserialize, Serialize};

/// Registered client application.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    /// Integer ID.
    pub id: usize,
    /// API resource URL.
    pub uri: String,
    /// URL to the SoundCloud.com page
    pub permalink_url: String,
    /// URL to an external site.
    pub external_url: String,
    /// Username of the app creator.
    pub creator: Option<String>,
}
