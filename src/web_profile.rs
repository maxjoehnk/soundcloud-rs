// Copyright (c) 2020
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::stream::BoxStream;
use serde::Deserialize;

use crate::client::Client;
use crate::error::Result;
use crate::streaming_api::{StreamingApi};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum WebProfileKind {
    #[serde(rename = "web-profile")]
    WebProfile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebProfile {
    pub kind: WebProfileKind,
    pub id: usize,
    pub service: String,
    pub title: String,
    pub url: String,
    pub username: Option<String>,
    pub created_at: String,
}

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
