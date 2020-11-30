use serde::{Deserialize, Serialize};

/// Registered user.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// Integer ID.
    pub id: usize,
    /// Permalink of the resource.
    pub permalink: String,
    /// Username.
    pub username: String,
    /// API resource URL.
    pub uri: String,
    /// URL to the SoundCloud.com page.
    pub permalink_url: String,
    /// URL to a JPEG image.
    pub avatar_url: String,
    /// Country.
    pub country: Option<String>,
    /// First and last name.
    pub full_name: Option<String>,
    /// City.
    pub city: Option<String>,
    /// Description, written by the user.
    pub description: Option<String>,
    /// Discogs name.
    #[serde(rename = "discogs-name")]
    pub discogs_name: Option<String>, // discogs-name
    /// MySpace name.
    #[serde(rename = "myspace-name")]
    pub myspace_name: Option<String>, // myspace-name
    /// URL to a website.
    pub website: Option<String>,
    /// Custom title for the website.
    #[serde(rename = "website-title")]
    pub website_title: Option<String>, // website-title
    /// Online status.
    pub online: Option<bool>,
    /// Number of public tracks.
    pub track_count: Option<usize>,
    /// Number of public playlists.
    pub playlist_count: Option<usize>,
    /// Number of followers.
    pub followers_count: Option<usize>,
    /// Number of followed users.
    pub followings_count: Option<usize>,
    /// Number of favorited public tracks.
    pub public_favorites_count: Option<usize>,
    // pub avatar_data …
}
