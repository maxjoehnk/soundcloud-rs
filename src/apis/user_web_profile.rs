use futures::stream::BoxStream;

use crate::client::Client;
use crate::error::Result;
use crate::models::WebProfile;
use crate::streaming_api::StreamingApi;

/// Provides access to operations available for a user's web profiles
pub struct WebProfiles {
    client: Client,
    user_id: usize,
}

impl WebProfiles {
    /// create a new instance of a souncloud user's web profiles
    pub fn new(client: Client, user_id: usize) -> Self {
        WebProfiles { client, user_id }
    }
}

impl StreamingApi for WebProfiles {
    type Model = WebProfile;

    fn path(&self) -> String {
        format!("/users/{}/web-profiles", self.user_id)
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<Result<Self::Model>> {
        self.client.get_stream(url, pages)
    }
}
