// Copyright (c) 2016, Mikkel Kroman <mk@uplink.io>
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::app::App;
use crate::client::Client;
use crate::error::{Error, Result};
use crate::user::User;

#[derive(Debug)]
pub enum Filter {
    All,
    Public,
    Private,
}

impl str::FromStr for Filter {
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

/// Uploaded track.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    /// Integer ID.
    pub id: u64,
    /// Time of which the track was uploaded, as an unparsed string.
    pub created_at: String,
    /// User ID of the uploader.
    pub user_id: u64,
    /// Small representation of the uploaders user.
    pub user: User,
    /// Title.
    pub title: String,
    /// Permalink of the resource.
    pub permalink: String,
    /// URL to the SoundCloud.com page.
    pub permalink_url: String,
    /// API resource URL.
    pub uri: String,
    /// Sharing status.
    pub sharing: String,
    /// Who can embed this track.
    pub embeddable_by: String,
    /// External purchase link.
    pub purchase_url: Option<String>,
    /// URL to a JPEG image.
    pub artwork_url: Option<String>,
    /// HTML description.
    pub description: Option<String>,
    /// Representation of a labels user.
    pub label: Option<serde_json::Value>,
    /// Duration in milliseconds.
    pub duration: u64,
    /// Genre.
    pub genre: Option<String>,
    /// List of tags.
    pub tags: Option<String>,
    /// Label user ID.
    pub label_id: Option<u64>,
    /// Label user name.
    pub label_name: Option<String>,
    /// Release number.
    pub release: Option<String>,
    /// Day of the release.
    pub release_day: Option<u64>,
    /// Month of the release.
    pub release_month: Option<u64>,
    /// Year of the release.
    pub release_year: Option<u64>,
    /// If the track is available for stream via the API.
    pub streamable: bool,
    /// If the track is available for download.
    pub downloadable: bool,
    /// Purchase title.
    pub purchase_title: Option<String>,
    /// Encoding state.
    pub state: String,
    /// Creative common license.
    pub license: String,
    /// Track type.
    pub track_type: Option<String>,
    /// URL to waveform PNG image.
    pub waveform_url: String,
    /// URL to original file.
    pub download_url: Option<String>,
    /// URL to 128kbps mp3 stream.
    pub stream_url: Option<String>,
    /// External video link.
    pub video_url: Option<String>,
    /// Beats per minute.
    pub bpm: Option<u64>,
    /// Commentable.
    pub commentable: bool,
    /// ISRC.
    pub isrc: Option<String>,
    /// Key.
    pub key_signature: Option<String>,
    /// Number of comments.
    pub comment_count: Option<u64>,
    /// Number of downloads.
    pub download_count: Option<u64>,
    /// Number of playbacks.
    pub playback_count: Option<u64>,
    /// Number of times favorited.
    pub favoritings_count: Option<u64>,
    /// Original upload format.
    pub original_format: String,
    /// Original upload size.
    pub original_content_size: u64,
    /// Application the track was uploaded with.
    pub created_with: Option<App>,
    /// Binary data of the audio file. Only for uploading.
    pub asset_data: Option<Vec<u8>>,
    /// Binary data of the artwork image. Only for uploading.
    pub artwork_data: Option<Vec<u8>>,
    /// User favorite.
    pub user_favorite: Option<bool>,
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
    types: Option<String>
}

#[derive(Debug)]
pub struct SingleTrackRequestBuilder<'a> {
    client: &'a Client,
    pub id: usize,
}

impl<'a> SingleTrackRequestBuilder<'a> {
    /// Constructs a new track request.
    pub fn new(client: &'a Client, id: usize) -> SingleTrackRequestBuilder {
        SingleTrackRequestBuilder {
            client,
            id,
        }
    }

    /// Sends the request and return the tracks.
    pub async fn get(&mut self) -> Result<Track> {
        let no_params: Option<&[(&str, &str)]> = None;
        let response = self.client.get(&format!("/tracks/{}", self.id), no_params).await?;
        let track: Track = response.json().await?;

        Ok(track)
    }

    pub fn request_url(&self) -> Url {
        Url::parse(&format!("https://{}/tracks/{}", super::API_HOST, self.id)).unwrap()
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
        where S: AsRef<str> {
        self.query = query.map(|s| s.as_ref().to_owned());
        self
    }

    /// Sets the tags filter, which will only return tracks with a matching tag.
    pub fn tags<I, T>(&'a mut self, tags: Option<I>) -> &mut TrackRequestBuilder
        where I: AsRef<[T]>, T: AsRef<str> {
        self.tags = tags.map(|s| {
            let tags_as_ref: Vec<_> = s.as_ref().iter().map(T::as_ref).collect();
            tags_as_ref.join(",")
        });
        self
    }

    pub fn genres<I, T>(&'a mut self, genres: Option<I>) -> &mut TrackRequestBuilder
        where I: AsRef<[T]>, T: AsRef<str> {
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

    /// Performs the request and returns a list of tracks if there are any results, None otherwise,
    /// or an error if one occurred.
    pub async fn get(&mut self) -> Result<Vec<Track>> {
        use serde_json::Value;

        let response = self.client.get("/tracks", Some(self.request_params())).await?;
        let track_list: Value = response.json().await?;

        if let Some(track_list) = track_list.as_array() {
            let tracks: Vec<Track> = track_list
                .iter().map(|t| serde_json::from_value::<Track>(t.clone()).unwrap()).collect();

            Ok(tracks)
        }else {
            Err(Error::ApiError("expected response to be an array".to_owned()))
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

impl PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        other.id == self.id
    }
}
