use futures::stream::BoxStream;

use crate::error::Result;
use crate::models::User;
use crate::streaming_api::StreamingApi;
use crate::Client;

/// Provides access to operations available for a track's likers
pub struct TrackLikers {
    client: Client,
    track_id: usize,
}

impl TrackLikers {
    /// create a new instance of a souncloud track's likers
    pub fn new(client: Client, track_id: usize) -> Self {
        TrackLikers { client, track_id }
    }
}

impl StreamingApi for TrackLikers {
    type Model = User;

    fn path(&self) -> String {
        format!("/tracks/{}/favoriters", self.track_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}
