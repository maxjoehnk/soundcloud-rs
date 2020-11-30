use futures::stream::BoxStream;

use crate::error::Result;
use crate::models::User;
use crate::streaming_api::StreamingApi;
use crate::Client;

/// Provides access to operations available for a user's followers
pub struct Followers {
    client: Client,
    user_id: usize,
}

impl Followers {
    /// create a new instance of a souncloud user's followers
    pub fn new(client: Client, user_id: usize) -> Self {
        Followers { client, user_id }
    }
}

impl StreamingApi for Followers {
    type Model = User;

    fn path(&self) -> String {
        format!("/users/{}/followers", self.user_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}
