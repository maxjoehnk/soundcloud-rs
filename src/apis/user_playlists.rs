use futures::stream::BoxStream;

use crate::error::Result;
use crate::models::Playlist;
use crate::streaming_api::StreamingApi;
use crate::Client;

/// Provides access to operations available for a user's playlists
pub struct Playlists {
    client: Client,
    user_id: usize,
}

impl StreamingApi for Playlists {
    type Model = Playlist;

    fn path(&self) -> String {
        format!("/users/{}/playlists", self.user_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}

impl Playlists {
    /// create a new instance of a souncloud user's playlists
    pub fn new(client: Client, user_id: usize) -> Self {
        Playlists { client, user_id }
    }
}
