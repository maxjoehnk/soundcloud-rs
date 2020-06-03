// Copyright (c) 2016, Mikkel Kroman <mk@uplink.io>
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Borrow;

use futures::stream::TryStreamExt;
use futures::io::AsyncWrite;
use url::Url;

use crate::error::{Error, Result};
use crate::playlist::{Playlist, PlaylistRequestBuilder, SinglePlaylistRequestBuilder};
use crate::track::{SingleTrackRequestBuilder, Track, TrackRequestBuilder};
use crate::user::{SingleUserRequestBuilder, UserRequestBuilder};

#[derive(Debug)]
pub struct Client {
    client_id: String,
    auth_token: Option<String>,
    http_client: reqwest::Client,
}

impl Client {
    /// Constructs a new `Client` with the provided `client_id`.
    ///
    /// # Examples
    ///
    /// ```
    /// use soundcloud::Client;
    ///
    /// let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    /// ```
    pub fn new(client_id: &str) -> Client {
        let client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build().unwrap();

        Client {
            client_id: client_id.to_owned(),
            http_client: client,
            auth_token: None,
        }
    }

    /// Returns the client id.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn authenticate_with_token(&mut self, token: String) {
        self.auth_token = Some(token);
    }

    /// Creates and sends a HTTP GET request to the API endpoint.
    ///
    /// A `client_id` parameter will automatically be added to the request.
    ///
    /// Returns the HTTP response on success, an error otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Read;
    /// use soundcloud::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    ///   let response = client.get("/resolve", Some(&[("url",
    ///   "https://soundcloud.com/firepowerrecs/afk-shellshock-kamikaze-promo-mix-lock-load-series-vol-20")])).await;
    ///
    ///   let buffer = response.unwrap().text().await.unwrap();
    ///
    ///   assert!(!buffer.is_empty());
    ///}
    /// ```
    pub async fn get<I, K, V>(&self, path: &str, params: Option<I>) -> reqwest::Result<reqwest::Response>
        where I: IntoIterator, I::Item: Borrow<(K, V)>, K: AsRef<str>, V: AsRef<str> 
    {
        let mut url = Url::parse(&format!("https://{}{}", super::API_HOST, path)).unwrap();

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("client_id", &self.client_id);

            if let Some(params) = params {
                query_pairs.extend_pairs(params);
            }
        }

        let mut headers = reqwest::header::HeaderMap::new();

        if self.auth_token.is_some() {
            let token = self.auth_token.clone().unwrap();
            headers.insert(reqwest::header::AUTHORIZATION, format!("OAuth {}", token).parse().unwrap());
        }

        let response = self.http_client
            .get(url)
            .headers(headers)
            .send()
            .await?;
        response.error_for_status()
    }

    /// Starts streaming the track provided in the track's `stream_url` to the `writer` if the track
    /// is streamable via the API.
    ///
    /// Returns:
    ///     Number of bytes written if the track was streamed successfully, an error otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use soundcloud::Client;
    /// use tokio::fs::File;
    /// use tokio_util::compat::Tokio02AsyncWriteCompatExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    ///   let path = Path::new("hi.mp3");
    ///   let track = client.tracks().id(263801976).get().await.unwrap();
    ///   let mut outfile = File::create(path).await.unwrap().compat_write();
    ///   let num_bytes = client.stream(&track, &mut outfile).await.unwrap();
    ///   assert!(num_bytes > 0);
    /// }
    /// ```
    pub async fn stream<W: AsyncWrite + Unpin>(&self, track: &Track, mut writer: W) -> Result<u64> {
        if !track.streamable {
            return Err(Error::TrackNotStreamable);
        }
        return self.read_url(&track.stream_url.as_ref().unwrap(), &mut writer).await;
    }

    /// Starts downloading the track provided in the tracks `download_url` to the `writer` if the track
    /// is downloadable via the API.
    ///
    /// Returns:
    ///     Number of bytes written if the track was downloaded successfully, an error otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use soundcloud::Client;
    /// use tokio::fs::File;
    /// use tokio_util::compat::Tokio02AsyncWriteCompatExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    ///   let path = Path::new("hi.mp3");
    ///   let track = client.tracks().id(263801976).get().await.unwrap();
    ///   let mut outfile = File::create(path).await.unwrap().compat_write();
    ///   let num_bytes = client.download(&track, &mut outfile).await.unwrap();
    ///   assert!(num_bytes > 0);
    /// }
    /// ```
    pub async fn download<W: AsyncWrite + Unpin>(&self, track: &Track, mut writer: W) -> Result<u64> {
        if !track.downloadable {
            return Err(Error::TrackNotDownloadable);
        }
        return self.read_url(&track.download_url.as_ref().unwrap(), &mut writer).await;
    }

    /// Copies the data provided from reading in the `url` to the `writer`
    /// if the track is streamable via the API.
    ///
    /// Returns:
    ///     number of bytes written if the resource's data was copied successfully, 
    ///     an error otherwise.
    ///
    /// ```
    async fn read_url<W: AsyncWrite + Unpin>(&self, url: &str, mut writer: W) -> Result<u64> {
        let url = self.parse_url(url);
        let mut response = self.http_client.get(url).send().await?;
        // Follow the redirect just this once.
        if let Some(header) = response.headers().get(reqwest::header::LOCATION).cloned() {
            let url = Url::parse(header.to_str()?).unwrap();
            response = self.http_client.get(url).send().await?;
        }
        let stream = response.bytes_stream();
        // convert the reqwest::Error into a futures::io::Error
        let stream = stream.map_err(|e|
            futures::io::Error::new(futures::io::ErrorKind::Other, e)
        ).into_async_read();

        let num_bytes = futures::io::copy(stream, &mut writer).await?;

        Ok(num_bytes)
    }

    /// Resolves any soundcloud resource and returns it as a `Url`.
    pub async fn resolve(&self, url: &str) -> Result<Url> {
        let response = self.get("/resolve", Some(&[("url", url)])).await?;

        if let Some(header) = response.headers().get(reqwest::header::LOCATION) {
            Ok(Url::parse(header.to_str()?).unwrap())
        } else {
            Err(Error::ApiError("expected location header".to_owned()))
        }
    }

    /// Returns a builder for a single track-by-id request.
    ///
    /// # Examples
    ///
    /// ```
    /// use soundcloud::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    ///   let track = client.track(262681089).get().await;
    ///
    ///   assert_eq!(track.unwrap().id, 262681089);
    /// }
    /// ```
    pub fn track(&self, id: usize) -> SingleTrackRequestBuilder {
        SingleTrackRequestBuilder::new(self, id)
    }

    /// Returns a builder for searching tracks with multiple criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// use soundcloud::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    ///   let tracks = client.tracks().genres(Some(["HipHop"])).get().await;
    ///
    ///   assert!(tracks.unwrap().len() > 0);
    /// }
    /// ```
    pub fn tracks(&self) -> TrackRequestBuilder {
        TrackRequestBuilder::new(self)
    }

    /// Returns a builder for a single playlist-by-id request.
    ///
    /// # Examples
    ///
    /// ```
    /// use soundcloud::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    ///   let playlist = client.playlist(965640322).get().await;
    ///
    ///   assert_eq!(playlist.unwrap().id, 965640322);
    /// }
    /// ```
    pub fn playlist(&self, id: usize) -> SinglePlaylistRequestBuilder {
        SinglePlaylistRequestBuilder::new(self, id)
    }

    /// Returns a builder for searching playlists with multiple criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// use soundcloud::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new(env!("SOUNDCLOUD_CLIENT_ID"));
    ///   let playlists = client.playlists().query("Monstercat").get().await;
    ///
    ///   assert!(playlists.unwrap().len() > 0);
    /// }
    /// ```
    pub fn playlists(&self) -> PlaylistRequestBuilder {
        PlaylistRequestBuilder::new(self)
    }

    /// Returns list of playlists of the authenticated user
    pub async fn my_playlists(&self) -> Result<Vec<Playlist>> {
        let params = Some(vec![("limit", "500")]);
        let res = self.get("/me/playlists", params).await?;
        let playlists: Vec<Playlist> = res.json().await?;
        Ok(playlists)
    }

    /// Returns a builder for searching users
    pub fn users(&self) -> UserRequestBuilder {
        UserRequestBuilder::new(self)
    }

    pub async fn likes(&self) -> Result<Vec<Track>> {
        let params = Some(vec![("limit", "500")]);
        let res = self.get("/me/favorites", params).await?;
        let likes: Vec<Track> = res.json().await?;
        Ok(likes)
    }

    /// Returns details about the given user
    pub fn user(&self, user_id: usize) -> SingleUserRequestBuilder {
        SingleUserRequestBuilder::new(self, user_id)
    }

    /// Parses a string and returns a url with the client_id query parameter set.
    fn parse_url<S: AsRef<str>>(&self, url: S) -> Url {
        let mut url = Url::parse(url.as_ref()).unwrap();
        url.query_pairs_mut().append_pair("client_id", &self.client_id);
        url
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    fn client() -> Client {
        Client::new(env!("SOUNDCLOUD_CLIENT_ID"))
    }

    fn authenticated_client() -> Client {
        let mut client = client();
        client.authenticate_with_token(env!("SOUNDCLOUD_AUTH_TOKEN").to_owned());

        client
    }

    #[tokio::test]
    async fn test_fetch_my_playlists() {
        let client = authenticated_client();
        assert!(client.my_playlists().await.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_fetch_likes() {
        let client = authenticated_client();
        assert!(client.likes().await.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_resolve_track() {
        let result = client().resolve("https://soundcloud.com/djmaksgermany/invites-feat-maks-warm-up-mix").await;

        assert_eq!(result.unwrap(),
                   Url::parse(&format!("https://api.soundcloud.com/tracks/330733497?client_id={}",
                                       env!("SOUNDCLOUD_CLIENT_ID"))).unwrap());
    }

    #[tokio::test]
    async fn test_get_tracks() {
        let result = client().tracks().query(Some("monstercat")).get().await;

        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_get_track() {
        let track = client().tracks().id(18201932).get().await.unwrap();

        assert_eq!(track.id, 18201932);
    }

    #[tokio::test]
    async fn test_get_playlists() {
        let result = client().playlists().query("monstercat").get().await;

        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_get_playlist() {
        let playlist = client().playlist(965640322).get().await.unwrap();

        assert_eq!(playlist.id, 965640322);
    }

    #[tokio::test]
    async fn test_download() {
        use tokio::fs::{File, remove_file};
        use tokio_util::compat::Tokio02AsyncWriteCompatExt;

        let client = client();
        let path = format!("hi.mp3");
        let track = client.tracks().id(263801976).get().await.unwrap();
        let mut outfile = File::create(&path).await.unwrap().compat_write();

        let num_bytes = client.download(&track, &mut outfile).await.unwrap();
        assert!(num_bytes > 0);
        let _ = remove_file(path).await;
    }

    #[tokio::test]
    async fn test_stream() {
        use tokio::fs::{File, remove_file};
        use tokio_util::compat::Tokio02AsyncWriteCompatExt;

        let client = client();
        let path = format!("test.mp3");
        let track = client.tracks().id(263801976).get().await.unwrap();
        let mut outfile = File::create(&path).await.unwrap().compat_write();

        let num_bytes = client.stream(&track, &mut outfile).await.unwrap();
        assert!(num_bytes > 0);
        let _ = remove_file(path).await;
    }

    #[tokio::test]
    async fn test_get_user() {
        let user = client().user(8553751).get().await.unwrap();

        assert_eq!(user.id, 8553751);
    }

    #[tokio::test]
    async fn test_get_user_tracks() {
        let tracks = client().user(8553751).tracks().await.unwrap();

        assert!(tracks.len() > 0);
    }

    #[tokio::test]
    async fn test_get_user_playlists() {
        let playlists = client().user(8553751).playlists().await.unwrap();

        assert!(playlists.len() > 0);
    }

    #[tokio::test]
    async fn test_get_user_from_permalink() {
        let user = client().users().permalink("west1ne").await.unwrap().get().await.unwrap();

        assert_eq!(user.id, 7466893);
    }

    #[tokio::test]
    async fn test_get_user_tracks_from_permalink() {
        let tracks = client().users().permalink("west1ne").await.unwrap().tracks().await;

        assert!(tracks.unwrap().len() > 0);
    }
}
