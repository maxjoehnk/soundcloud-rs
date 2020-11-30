use crate::models::{Track, User};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlaylistType {
    Single,
    Album,
    Ep,
    Compilation,
    #[serde(other)]
    Playlist,
}

impl Default for PlaylistType {
    fn default() -> Self {
        PlaylistType::Playlist
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum PlaylistKind {
    #[serde(rename = "playlist")]
    Playlist,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum PlaylistSharing {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Playlist {
    pub duration: u64,
    pub release_day: Option<i32>,
    pub permalink_url: String,
    pub permalink: String,
    pub playlist_type: Option<PlaylistType>,
    pub purchase_url: Option<String>,
    pub description: Option<String>,
    pub uri: String,
    pub track_count: u64,
    pub user_id: u64,
    pub kind: PlaylistKind,
    pub title: String,
    pub id: u64,
    #[serde(default)]
    pub tracks: Option<Vec<Track>>,
    pub user: User,
    pub artwork_url: Option<String>,
}
