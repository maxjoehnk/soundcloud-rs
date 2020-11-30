use crate::apis::{Comments, Followers, Followings, Likes, Playlists, Tracks, WebProfiles};
use crate::error::{Error, Result};
use crate::models::User;
use crate::Client;

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
    where
        S: AsRef<str>,
    {
        self.query = query.map(|s| s.as_ref().to_owned());
        self
    }

    /// Returns a builder for a user request
    pub fn id(&self, id: usize) -> SingleUserRequestBuilder {
        SingleUserRequestBuilder {
            client: self.client,
            id,
        }
    }

    /// Creates a user request builder by resolving a user's unique permalink to
    /// their user id.
    ///
    /// Returns:
    ///     a builder for a user request
    pub async fn permalink(&self, permalink: &str) -> Result<SingleUserRequestBuilder<'a>> {
        let permalink_url = &format!("https://soundcloud.com/{}", permalink);
        let resource_url = self.client.resolve(permalink_url).await?;
        let id = resource_url
            .path_segments()
            .map(|c| c.collect::<Vec<_>>())
            .unwrap()
            .pop()
            .unwrap();
        let id = usize::from_str_radix(id, 10).unwrap();
        Ok(SingleUserRequestBuilder {
            client: self.client,
            id,
        })
    }

    /// Performs the request and returns a list of users or an error if one occurred.
    pub async fn get(&mut self) -> Result<Vec<User>> {
        use serde_json::Value;

        let response = self
            .client
            .get("/users", Some(self.request_params()))
            .await?;
        let user_list: Value = response.json().await?;

        if let Some(user_list) = user_list.as_array() {
            let users: Vec<User> = user_list
                .iter()
                .map(|t| serde_json::from_value::<User>(t.clone()).unwrap())
                .collect();

            Ok(users)
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

impl<'a> SingleUserRequestBuilder<'a> {
    /// Creates a new user request builder, with no set parameters.
    pub fn new(client: &'a Client, id: usize) -> SingleUserRequestBuilder<'a> {
        SingleUserRequestBuilder { client, id }
    }

    /// Retrieve all tracks uploaded by the user
    ///
    /// Returns:
    ///     an instance of Tracks
    pub fn tracks(&self) -> Tracks {
        Tracks::new(self.client.clone(), self.id)
    }

    /// Retrieve all tracks liked by the user
    ///
    /// Returns:
    ///     an instance of Likes
    pub fn likes(&mut self) -> Likes {
        Likes::new(self.client.clone(), self.id)
    }

    /// Retrieve all playlists uploaded by the user
    ///
    /// Returns:
    ///     an instance of Playlists
    pub fn playlists(&mut self) -> Playlists {
        Playlists::new(self.client.clone(), self.id)
    }

    /// Retrieve all comments for this user
    ///
    /// Returns:
    ///     an instance of Comments
    pub fn comments(&mut self) -> Comments {
        Comments::user(self.client.clone(), self.id)
    }

    /// Retrieve all users this user follows
    ///
    /// Returns:
    ///     an instance of Followings
    pub fn followings(&mut self) -> Followings {
        Followings::new(self.client.clone(), self.id)
    }

    /// Retrieve all this user's followers
    ///
    /// Returns:
    ///     an instance of Followers
    pub fn followers(&mut self) -> Followers {
        Followers::new(self.client.clone(), self.id)
    }

    /// Retrieve all this user's web profiles
    ///
    /// Returns:
    ///     an instance of WebProfiles
    pub fn web_profiles(&mut self) -> WebProfiles {
        WebProfiles::new(self.client.clone(), self.id)
    }

    /// Retrieve a SoundCloud user
    ///
    /// Returns:
    ///     User data in JSON format
    pub async fn get(&mut self) -> Result<User> {
        let no_params: Option<&[(&str, &str)]> = None;
        let response = self
            .client
            .get(&format!("/users/{}", self.id), no_params)
            .await?;
        let user: User = response.json().await?;

        Ok(user)
    }
}
