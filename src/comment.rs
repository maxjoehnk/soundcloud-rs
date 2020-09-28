use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Result;
use crate::page::PageOptions;
use crate::user::User;

/// User comment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    /// Integer ID.
    pub id: usize,
    /// API resource URL.
    pub uri: String,
    /// Time of creation, as an unparsed string.
    pub created_at: String,
    /// HTML comment body.
    pub body: String,
    /// Associated timestamp in milliseconds.
    pub timestamp: Option<usize>,
    /// User ID of the commenter.
    pub user_id: usize,
    /// Small representation of the commenters user.
    pub user: User,
    /// The track ID of the related track.
    pub track_id: usize,
}

/// Provides access to operations available for a user's comments
pub struct Comments {
    client: Client,
    user_id: usize,
}

impl Comments {
    /// create a new instance of a souncloud user's comments
    pub fn new(client: Client, user_id: usize) -> Self {
        Comments { client, user_id }
    }

    fn path(&self, extra: &str) -> String {
        format!("/users/{}/comments{}", self.user_id, extra)
    }

    /// Return a stream of all comments for this user
    pub fn iter(&self, options: &PageOptions) -> BoxStream<Result<Comment>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.client.get_stream(&uri.join("?"), u64::MAX)
    }

    /// Return a stream of comments for this user limited to the first num_pages pages
    pub fn get(&self, options: &PageOptions, num_pages: u64) -> BoxStream<Result<Comment>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.client.get_stream(&uri.join("?"), num_pages)
    }
}
