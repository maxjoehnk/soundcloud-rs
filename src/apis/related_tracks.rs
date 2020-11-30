use futures::stream::BoxStream;

use crate::error::Result;
use crate::models::Track;
use crate::streaming_api::StreamingApi;
use crate::Client;

/// Provides access to operations available for a track's related tracks
pub struct RelatedTracks {
    client: Client,
    track_id: usize,
}

impl RelatedTracks {
    /// create a new instance of a souncloud track's related tracks
    pub fn new(client: Client, track_id: usize) -> Self {
        RelatedTracks { client, track_id }
    }
}

impl StreamingApi for RelatedTracks {
    type Model = Track;

    fn path(&self) -> String {
        format!("/tracks/{}/related", self.track_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}
