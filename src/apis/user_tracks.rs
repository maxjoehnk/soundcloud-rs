use futures::stream::BoxStream;

use crate::error::Result;
use crate::models::Track;
use crate::streaming_api::StreamingApi;
use crate::Client;

/// Provides access to operations available for a user's tracks
pub struct Tracks {
    client: Client,
    user_id: usize,
}

impl Tracks {
    /// create a new instance of a souncloud user's tracks
    pub fn new(client: Client, user_id: usize) -> Self {
        Tracks { client, user_id }
    }
}

impl StreamingApi for Tracks {
    type Model = Track;

    fn path(&self) -> String {
        format!("/users/{}/tracks", self.user_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}
