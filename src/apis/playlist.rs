use crate::client::Client;
use crate::error::{Error, Result};
use crate::models::Playlist;

#[derive(Debug)]
pub struct PlaylistRequestBuilder<'a> {
    client: &'a Client,
    query: Option<String>,
}

#[derive(Debug)]
pub struct SinglePlaylistRequestBuilder<'a> {
    client: &'a Client,
    pub id: usize,
}

impl<'a> SinglePlaylistRequestBuilder<'a> {
    /// Constructs a new track request.
    pub fn new(client: &'a Client, id: usize) -> SinglePlaylistRequestBuilder {
        SinglePlaylistRequestBuilder { client, id }
    }

    /// Sends the request and return the tracks.
    pub async fn get(&mut self) -> Result<Playlist> {
        let no_params: Option<&[(&str, &str)]> = None;
        let response = self
            .client
            .get(&format!("/playlists/{}", self.id), no_params)
            .await?;
        let track: Playlist = response.json().await?;

        Ok(track)
    }
}

impl<'a> PlaylistRequestBuilder<'a> {
    /// Creates a new playlist request builder, with no set parameters.
    pub fn new(client: &'a Client) -> Self {
        PlaylistRequestBuilder {
            client,
            query: None,
        }
    }

    /// Sets the search query filter, which will only return playlists with a matching query.
    pub fn query<S>(&'a mut self, query: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.query = Some(query.as_ref().to_owned());
        self
    }

    /// Returns a builder for a single playlist.
    pub fn id(&'a mut self, id: usize) -> SinglePlaylistRequestBuilder {
        SinglePlaylistRequestBuilder {
            client: &self.client,
            id,
        }
    }

    /// Performs the request and returns a list of playlists or an error if one occurred.
    pub async fn get(&mut self) -> Result<Vec<Playlist>> {
        use serde_json::Value;

        let response = self
            .client
            .get("/playlists", Some(self.request_params()))
            .await?;
        let playlist_list: Value = response.json().await?;

        if let Some(playlist_list) = playlist_list.as_array() {
            let playlists: Vec<Playlist> = playlist_list
                .iter()
                .map(|p| serde_json::from_value::<Playlist>(p.clone()).unwrap())
                .collect();

            Ok(playlists)
        } else {
            Err(Error::ApiError(
                "expected response to be an array".to_owned(),
            ))
        }
    }

    fn request_params(&self) -> Vec<(&str, String)> {
        let mut result = vec![];

        if let Some(ref query) = self.query {
            result.push(("q", query.clone()));
        }

        result
    }
}
