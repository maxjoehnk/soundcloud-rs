use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Result;
use crate::track::Track;
use crate::playlist::Playlist;

/// Registered user.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// Integer ID.
    pub id: usize,
    /// Permalink of the resource.
    pub permalink: String,
    /// Username.
    pub username: String,
    /// API resource URL.
    pub uri: String,
    /// URL to the SoundCloud.com page.
    pub permalink_url: String,
    /// URL to a JPEG image.
    pub avatar_url: String,
    /// Country.
    pub country: Option<String>,
    /// First and last name.
    pub full_name: Option<String>,
    /// City.
    pub city: Option<String>,
    /// Description, written by the user.
    pub description: Option<String>,
    /// Discogs name.
    #[serde(rename="discogs-name")]
    pub discogs_name: Option<String>, // discogs-name
    /// MySpace name.
    #[serde(rename="myspace-name")]
    pub myspace_name: Option<String>, // myspace-name
    /// URL to a website.
    pub website: Option<String>,
    /// Custom title for the website.
    #[serde(rename="website-title")]
    pub website_title: Option<String>, // website-title
    /// Online status.
    pub online: Option<bool>,
    /// Number of public tracks.
    pub track_count: Option<usize>,
    /// Number of public playlists.
    pub playlist_count: Option<usize>,
    /// Number of followers.
    pub followers_count: Option<usize>,
    /// Number of followed users.
    pub followings_count: Option<usize>,
    /// Number of favorited public tracks.
    pub public_favorites_count: Option<usize>,
    // pub avatar_data …
}

#[derive(Debug)]
pub struct UserRequestBuilder<'a> {
    client: &'a Client,
    query: Option<String>,
}

#[derive(Debug)]
pub struct SingleUserRequestBuilder<'a> {
    client: &'a Client,
    pub id: usize,
}

impl<'a> UserRequestBuilder<'a> {
    /// Creates a new user request builder, with no set parameters.
    pub fn new(client: &'a Client) -> UserRequestBuilder<'a> {
        UserRequestBuilder {
            client,
            query: None,
        }
    }

    /// Sets the search query filter, which will only return tracks with a matching query.
    pub fn query<S>(&'a mut self, query: Option<S>) -> &mut UserRequestBuilder
        where S: AsRef<str> {
        self.query = query.map(|s| s.as_ref().to_owned());
        self
    }

    /// Returns a builder for a single track.
    pub fn id(&self, id: usize) -> SingleUserRequestBuilder {
        SingleUserRequestBuilder {
            client: self.client,
            id,
        }
    }

    /// Returns a builder for a single track.
    pub async fn permalink(&self, permalink: &str) -> Result<SingleUserRequestBuilder<'a>> {
        let permalink_url = &format!("https://soundcloud.com/{}", permalink);
        let resource_url = self.client.resolve(permalink_url).await?;
        let id = resource_url.path_segments()
            .map(|c| {
                c.collect::<Vec<_>>()
            }).unwrap().pop().unwrap();
        let id = usize::from_str_radix(id, 10).unwrap();
        Ok(SingleUserRequestBuilder {
            client: self.client,
            id,
        })
    }
}

impl<'a> SingleUserRequestBuilder<'a> {
    /// Creates a new user request builder, with no set parameters.
    pub fn new(client: &'a Client, id: usize) -> SingleUserRequestBuilder<'a> {
        SingleUserRequestBuilder {
            client,
            id,
        }
    }

    /// Retrieve all tracks uploaded by the artist
    ///
    /// Returns:
    ///     a list of tracks or an error if one occurred.
    pub async fn tracks(&mut self) -> Result<Vec<Track>> {
        let path = &format!("/users/{}/tracks", self.id.to_string());
        let no_params: Option<&[(&str, &str)]> = None;
        let response = self.client.get(&path, no_params).await?;
        let tracks: Vec<Track> = response.json().await?;

        Ok(tracks)
    }

    /// Retrieve all playlists uploaded by the user
    ///
    /// Returns:
    ///     a list of playlists or an error if one occurred.
    pub async fn playlists(&mut self) -> Result<Vec<Playlist>> {
        let path = format!("/users/{}/playlists", self.id.to_string());
        let no_params: Option<&[(&str, &str)]> = None;
        let response = self.client.get(&path, no_params).await?;
        let playlists: Vec<Playlist> = response.json().await?;

        Ok(playlists)
    }

    /// Retrieve a SoundCloud user
    ///
    /// Returns:
    ///     User data in JSON format
    pub async fn get(&mut self) -> Result<User> {
        let no_params: Option<&[(&str, &str)]> = None;
        let response = self.client.get(&format!("/users/{}", self.id), no_params).await?;
        let user: User = response.json().await?;

        Ok(user)
    }
}
