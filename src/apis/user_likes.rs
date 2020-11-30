use futures::stream::BoxStream;

use crate::error::Result;
use crate::models::Track;
use crate::streaming_api::StreamingApi;
use crate::Client;

/// Provides access to operations available for a user's liked tracks
pub struct Likes {
    client: Client,
    user_id: usize,
}

impl Likes {
    /// create a new instance of a souncloud user's likes
    pub fn new(client: Client, user_id: usize) -> Self {
        Likes { client, user_id }
    }
}

impl StreamingApi for Likes {
    type Model = Track;

    fn path(&self) -> String {
        format!("/users/{}/favorites", self.user_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}
