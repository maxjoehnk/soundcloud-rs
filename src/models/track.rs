use crate::models::{App, User};
use serde::{Deserialize, Serialize};

/// Uploaded track.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    /// Integer ID.
    pub id: u64,
    /// Time of which the track was uploaded, as an unparsed string.
    pub created_at: String,
    /// User ID of the uploader.
    pub user_id: u64,
    /// Small representation of the uploaders user.
    pub user: User,
    /// Title.
    pub title: String,
    /// Permalink of the resource.
    pub permalink: String,
    /// URL to the SoundCloud.com page.
    pub permalink_url: String,
    /// API resource URL.
    pub uri: String,
    /// Sharing status.
    pub sharing: String,
    /// Who can embed this track.
    pub embeddable_by: String,
    /// External purchase link.
    pub purchase_url: Option<String>,
    /// URL to a JPEG image.
    pub artwork_url: Option<String>,
    /// HTML description.
    pub description: Option<String>,
    /// Representation of a labels user.
    pub label: Option<serde_json::Value>,
    /// Duration in milliseconds.
    pub duration: u64,
    /// Genre.
    pub genre: Option<String>,
    /// List of tags.
    pub tags: Option<String>,
    /// Label user ID.
    pub label_id: Option<u64>,
    /// Label user name.
    pub label_name: Option<String>,
    /// Release number.
    pub release: Option<String>,
    /// Day of the release.
    pub release_day: Option<u64>,
    /// Month of the release.
    pub release_month: Option<u64>,
    /// Year of the release.
    pub release_year: Option<u64>,
    /// If the track is available for stream via the API.
    pub streamable: bool,
    /// If the track is available for download.
    pub downloadable: bool,
    /// Purchase title.
    pub purchase_title: Option<String>,
    /// Encoding state.
    pub state: String,
    /// Creative common license.
    pub license: String,
    /// Track type.
    pub track_type: Option<String>,
    /// URL to waveform PNG image.
    pub waveform_url: String,
    /// URL to original file.
    pub download_url: Option<String>,
    /// URL to 128kbps mp3 stream.
    pub stream_url: Option<String>,
    /// External video link.
    pub video_url: Option<String>,
    /// Beats per minute.
    pub bpm: Option<u64>,
    /// Commentable.
    pub commentable: bool,
    /// ISRC.
    pub isrc: Option<String>,
    /// Key.
    pub key_signature: Option<String>,
    /// Number of comments.
    pub comment_count: Option<u64>,
    /// Number of downloads.
    pub download_count: Option<u64>,
    /// Number of playbacks.
    pub playback_count: Option<u64>,
    /// Number of times favorited.
    pub favoritings_count: Option<u64>,
    /// Original upload format.
    pub original_format: String,
    /// Original upload size.
    pub original_content_size: u64,
    /// Application the track was uploaded with.
    pub created_with: Option<App>,
    /// Binary data of the audio file. Only for uploading.
    pub asset_data: Option<Vec<u8>>,
    /// Binary data of the artwork image. Only for uploading.
    pub artwork_data: Option<Vec<u8>>,
    /// User favorite.
    pub user_favorite: Option<bool>,
}

impl PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        other.id == self.id
    }
}
