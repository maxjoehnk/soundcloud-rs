use std::fmt;
use std::str::FromStr;

use crate::apis::{Comments, RelatedTracks, TrackLikers};
use crate::client::Client;
use crate::error::{Error, Result};
use crate::models::Track;

#[derive(Debug)]
pub enum Filter {
    All,
    Public,
    Private,
}

impl FromStr for Filter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Filter> {
        match s {
            "all" => Ok(Filter::All),
            "public" => Ok(Filter::Public),
            "private" => Ok(Filter::Private),
            _ => Err(Error::InvalidFilter(s.to_string())),
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Filter {
    pub fn to_str(&self) -> &str {
        match *self {
            Filter::All => "all",
            Filter::Public => "public",
            Filter::Private => "private",
        }
    }
}

#[derive(Debug)]
pub struct TrackRequestBuilder<'a> {
    client: &'a Client,
    query: Option<String>,
    tags: Option<String>,
    filter: Option<Filter>,
    license: Option<String>,
    ids: Option<Vec<usize>>,
    duration: Option<(usize, usize)>,
    bpm: Option<(usize, usize)>,
    genres: Option<String>,
    types: Option<String>,
}

#[derive(Debug)]
pub struct SingleTrackRequestBuilder<'a> {
    client: &'a Client,
    pub id: usize,
}

impl<'a> SingleTrackRequestBuilder<'a> {
    /// Constructs a new track request.
    pub fn new(client: &'a Client, id: usize) -> SingleTrackRequestBuilder {
        SingleTrackRequestBuilder { client, id }
    }

    /// Retrieve all comments for this track
    ///
    /// Returns:
    ///     an instance of Comments
    pub fn comments(&mut self) -> Comments {
        Comments::track(self.client.clone(), self.id)
    }

    /// Retrieve all tracks related to this track
    ///
    /// Returns:
    ///     an instance of RelatedTracks
    pub fn related_tracks(&mut self) -> RelatedTracks {
        RelatedTracks::new(self.client.clone(), self.id)
    }

    /// Retrieve all soundcloud users that like this track
    ///
    /// Returns:
    ///     an instance of Likers
    pub fn likers(&mut self) -> TrackLikers {
        TrackLikers::new(self.client.clone(), self.id)
    }

    /// Sends the request and return the tracks.
    pub async fn get(&mut self) -> Result<Track> {
        let no_params: Option<&[(&str, &str)]> = None;
        let response = self
            .client
            .get(&format!("/tracks/{}", self.id), no_params)
            .await?;
        let track: Track = response.json().await?;

        Ok(track)
    }
}

impl<'a> TrackRequestBuilder<'a> {
    /// Creates a new track request builder, with no set parameters.
    pub fn new(client: &'a Client) -> TrackRequestBuilder {
        TrackRequestBuilder {
            client,
            query: None,
            tags: None,
            filter: None,
            license: None,
            ids: None,
            duration: None,
            bpm: None,
            genres: None,
            types: None,
        }
    }

    /// Sets the search query filter, which will only return tracks with a matching query.
    pub fn query<S>(&'a mut self, query: Option<S>) -> &mut TrackRequestBuilder
    where
        S: AsRef<str>,
    {
        self.query = query.map(|s| s.as_ref().to_owned());
        self
    }

    /// Sets the tags filter, which will only return tracks with a matching tag.
    pub fn tags<I, T>(&'a mut self, tags: Option<I>) -> &mut TrackRequestBuilder
    where
        I: AsRef<[T]>,
        T: AsRef<str>,
    {
        self.tags = tags.map(|s| {
            let tags_as_ref: Vec<_> = s.as_ref().iter().map(T::as_ref).collect();
            tags_as_ref.join(",")
        });
        self
    }

    pub fn genres<I, T>(&'a mut self, genres: Option<I>) -> &mut TrackRequestBuilder
    where
        I: AsRef<[T]>,
        T: AsRef<str>,
    {
        self.genres = genres.map(|s| {
            let genres_as_ref: Vec<_> = s.as_ref().iter().map(T::as_ref).collect();
            genres_as_ref.join(",")
        });
        self
    }

    /// Sets whether to filter private or public tracks.
    pub fn filter(&'a mut self, filter: Option<Filter>) -> &mut TrackRequestBuilder {
        self.filter = filter;
        self
    }

    /// Sets the license filter.
    pub fn license<S: AsRef<str>>(&'a mut self, license: Option<S>) -> &mut TrackRequestBuilder {
        self.license = license.map(|s| s.as_ref().to_owned());
        self
    }

    /// Sets a list of track ids to look up.
    pub fn ids(&'a mut self, ids: Option<Vec<usize>>) -> &mut TrackRequestBuilder {
        self.ids = ids;
        self
    }

    /// Returns a builder for a single track.
    pub fn id(&'a mut self, id: usize) -> SingleTrackRequestBuilder {
        SingleTrackRequestBuilder {
            client: &self.client,
            id,
        }
    }

    /// Performs the request and returns a list of tracks or an error if one occurred.
    pub async fn get(&mut self) -> Result<Vec<Track>> {
        use serde_json::Value;

        let response = self
            .client
            .get("/tracks", Some(self.request_params()))
            .await?;
        let track_list: Value = response.json().await?;

        if let Some(track_list) = track_list.as_array() {
            let tracks: Vec<Track> = track_list
                .iter()
                .map(|t| serde_json::from_value::<Track>(t.clone()).unwrap())
                .collect();

            Ok(tracks)
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

        if let Some(ref tags) = self.tags {
            result.push(("tags", tags.clone()));
        }

        if let Some(ref filter) = self.filter {
            result.push(("filter", filter.to_str().to_owned()));
        }

        if let Some(ref ids) = self.ids {
            let ids_as_strings: Vec<String> = ids.iter().map(|id| format!("{}", id)).collect();
            result.push(("ids", ids_as_strings.join(",")));
        }

        if let Some(ref _duration) = self.duration {
            unimplemented!();
        }

        if let Some(ref _bpm) = self.bpm {
            unimplemented!();
        }

        if let Some(ref genres) = self.genres {
            result.push(("genres", genres.clone()));
        }

        if let Some(ref types) = self.types {
            result.push(("types", types.clone()));
        }

        result
    }
}
