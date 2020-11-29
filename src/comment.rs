use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Result;
use crate::user::User;
use crate::streaming_api::StreamingApi;

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

/// Provides access to operations available for comments
pub enum Comments {
    User {
        client: Client,
        user_id: usize,
    },
    Track {
        client: Client,
        track_id: usize,
    }
}

impl StreamingApi for Comments {
    type Model = Comment;

    fn path(&self) -> String {
        match self {
            Comments::Track { client: _, track_id } => {
                format!("/tracks/{}/comments", track_id)
            },
            Comments::User { client: _, user_id } => {
                format!("/users/{}/comments", user_id)
            }
        }
    }

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<'_, Result<Self::Model>> {
        let client = match self {
            Comments::Track { client, track_id: _ } => client,
            Comments::User { client, user_id: _ } => client,
        };
        client.get_stream(url, pages)
    }
}

impl Comments {
    /// create a new instance of a souncloud user's comments
    pub fn user(client: Client, user_id: usize) -> Self {
        Comments::User { client, user_id }
    }

    /// create a new instance of a souncloud track's comments
    pub fn track(client: Client, track_id: usize) -> Self {
        Comments::Track { client, track_id }
    }
}
