use futures::stream::BoxStream;

use crate::error::Result;
use crate::models::User;
use crate::streaming_api::StreamingApi;
use crate::Client;

/// Provides access to operations available for a user's followings
pub struct Followings {
    client: Client,
    user_id: usize,
}

impl Followings {
    /// create a new instance of a souncloud user's followings
    pub fn new(client: Client, user_id: usize) -> Self {
        Followings { client, user_id }
    }
}

impl StreamingApi for Followings {
    type Model = User;

    fn path(&self) -> String {
        format!("/users/{}/followings", self.user_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}
