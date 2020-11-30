use std::borrow::Borrow;

use futures::future::BoxFuture;
use futures::io::AsyncWrite;
use futures::prelude::*;
use futures::stream::{BoxStream, TryStreamExt};
use serde::de::DeserializeOwned;
use url::Url;

use crate::apis::{
    PlaylistRequestBuilder, SinglePlaylistRequestBuilder, SingleTrackRequestBuilder,
    SingleUserRequestBuilder, TrackRequestBuilder, UserRequestBuilder,
};
use crate::error::{Error, Result};
use crate::models::{Playlist, Track};
use crate::page::Page;

#[derive(Clone, Debug)]
pub struct Client {
    host: String,
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
            .build()
            .unwrap();

        Client {
            host: super::API_HOST.to_owned(),
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
    pub async fn get<I, K, V>(&self, path: &str, params: Option<I>) -> Result<reqwest::Response>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut url = Url::parse(&format!("{}", self.host.clone() + path))?;

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
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("OAuth {}", token).parse()?,
            );
        }

        let response = self.http_client.get(url).headers(headers).send().await?;
        response.error_for_status().map_err(Error::from)
    }

    pub fn get_stream<T>(&self, path: &str, num_pages: Option<u64>) -> BoxStream<Result<T>>
    where
        T: DeserializeOwned + 'static + Send,
    {
        unfold(
            self.clone(),
            self.get_pages(&path),
            num_pages.unwrap_or(u64::MAX),
        )
    }

    fn get_pages<T>(&self, path: &str) -> BoxFuture<Result<Page<T>>>
    where
        T: DeserializeOwned + 'static + Send,
    {
        self.get_page(&(self.host.clone() + path))
    }

    fn get_pages_url<T>(&self, url: &str) -> BoxFuture<Result<Page<T>>>
    where
        T: DeserializeOwned + 'static + Send,
    {
        self.get_page(url)
    }

    fn get_page<T>(&self, path: &str) -> BoxFuture<Result<Page<T>>>
    where
        T: DeserializeOwned + 'static + Send,
    {
        let mut url = Url::parse(path).unwrap();

        if !url.query_pairs().any(|(q, _)| q == "client_id") {
            url.query_pairs_mut()
                .append_pair("client_id", &self.client_id);
        }

        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(ref token) = self.auth_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("OAuth {}", token).parse().unwrap(),
            );
        }

        let response = self
            .http_client
            .get(url)
            .headers(headers)
            .send()
            .map_err(Error::from);

        Box::pin(response.and_then(move |response| response.json().map_err(Error::from)))
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
        self.read_url(&track.stream_url.as_ref().unwrap(), &mut writer)
            .await
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
    pub async fn download<W: AsyncWrite + Unpin>(
        &self,
        track: &Track,
        mut writer: W,
    ) -> Result<u64> {
        if !track.downloadable {
            return Err(Error::TrackNotDownloadable);
        }
        self.read_url(&track.download_url.as_ref().unwrap(), &mut writer)
            .await
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
        let url = self.parse_url(url)?;
        let mut response = self.http_client.get(url).send().await?;
        // Follow the redirect just this once.
        if let Some(header) = response.headers().get(reqwest::header::LOCATION).cloned() {
            let url = Url::parse(header.to_str()?).unwrap();
            response = self.http_client.get(url).send().await?;
        }
        let stream = response.bytes_stream();
        // convert the reqwest::Error into a futures::io::Error
        let stream = stream
            .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
            .into_async_read();

        let num_bytes = futures::io::copy(stream, &mut writer).await?;

        Ok(num_bytes)
    }

    /// Resolves any soundcloud resource and returns it as a `Url`.
    pub async fn resolve(&self, url: &str) -> Result<Url> {
        let response = self.get("/resolve", Some(&[("url", url)])).await?;

        if let Some(header) = response.headers().get(reqwest::header::LOCATION) {
            Ok(Url::parse(header.to_str()?)?)
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

    /// Returns details about the given user
    pub fn user(&self, user_id: usize) -> SingleUserRequestBuilder {
        SingleUserRequestBuilder::new(self, user_id)
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

    /// Parses a string and returns a url with the client_id query parameter set.
    fn parse_url<S: AsRef<str>>(&self, url: S) -> Result<Url> {
        let mut url = Url::parse(url.as_ref())?;
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id);
        Ok(url)
    }
}

/// "unfold" paginated results of a list of soundcloud entities
fn unfold<T>(
    client: Client,
    first: BoxFuture<Result<Page<T>>>,
    num_pages: u64,
) -> BoxStream<Result<T>>
where
    T: DeserializeOwned + 'static + Send,
{
    Box::pin(
        first
            .map_ok(move |page| {
                let count = 1;
                let mut items = page.collection;
                items.reverse();
                let link = page.next_href;
                stream::try_unfold(
                    (client, link, items, count),
                    move |(client, link, mut items, mut count)| async move {
                        match items.pop() {
                            Some(item) => Ok(Some((item, (client, link, items, count)))),
                            None => {
                                if count == num_pages {
                                    Ok(None)
                                } else {
                                    match link {
                                        Some(url) => {
                                            count += 1;
                                            let page = client.get_pages_url(&url).await?;
                                            let link = page.next_href;
                                            let mut items = page.collection;
                                            items.reverse();
                                            match items.pop() {
                                                Some(item) => {
                                                    Ok(Some((item, (client, link, items, count))))
                                                }
                                                None => Ok(None),
                                            }
                                        }
                                        None => Ok(None),
                                    }
                                }
                            }
                        }
                    },
                )
            })
            .try_flatten_stream(),
    )
}
