use crate::models::User;
use serde::{Deserialize, Serialize};

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
