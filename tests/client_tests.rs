use url::Url;

use futures::prelude::*;
use soundcloud::*;

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
    let result = client()
        .resolve("https://soundcloud.com/djmaksgermany/invites-feat-maks-warm-up-mix")
        .await;

    assert_eq!(
        result.unwrap(),
        Url::parse(&format!(
            "https://api.soundcloud.com/tracks/330733497?client_id={}",
            env!("SOUNDCLOUD_CLIENT_ID")
        ))
        .unwrap()
    );
}

#[tokio::test]
async fn test_get_tracks() {
    let result = client().tracks().query(Some("monstercat")).get().await;

    assert!(result.unwrap().len() > 0);
}

#[tokio::test]
async fn test_get_track() {
    let track = client().tracks().id(263801976).get().await.unwrap();

    assert_eq!(track.id, 263801976);
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
    use tokio::fs::{remove_file, File};
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
    use tokio::fs::{remove_file, File};
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
async fn test_get_users() {
    let users = client()
        .users()
        .query(Some("monstercat"))
        .get()
        .await
        .unwrap();

    assert!(users.len() > 0);
}

#[tokio::test]
async fn test_get_user_from_permalink() {
    let user = client()
        .users()
        .permalink("west1ne")
        .await
        .unwrap()
        .get()
        .await
        .unwrap();

    assert_eq!(user.id, 7466893);
}

#[tokio::test]
async fn test_get_first_page_user_tracks() {
    let tracks = client().user(7466893).tracks();
    let tracks: Vec<Track> = tracks
        .get(Default::default(), 1)
        .try_collect()
        .await
        .unwrap();

    assert!(tracks.len() > 0);
}

#[tokio::test]
async fn test_paginate_user_tracks() {
    let tracks = client().user(7466893).tracks();
    let tracks: Vec<Track> = tracks.iter(Default::default()).try_collect().await.unwrap();

    assert!(tracks.len() > 0);
}

#[tokio::test]
async fn test_user_web_profile() {
    let profiles = client().user(7466893).web_profiles();
    let profiles: Vec<WebProfile> = profiles
        .iter(Default::default())
        .try_collect()
        .await
        .unwrap();

    assert!(profiles.len() > 0);
}

#[tokio::test]
async fn test_user_playlists() {
    let playlists = client().user(7466893).playlists();
    let playlists: Vec<Playlist> = playlists
        .iter(Default::default())
        .try_collect()
        .await
        .unwrap();

    assert!(playlists.len() > 0);
}

#[tokio::test]
async fn test_user_comments() {
    let comments = client().user(7466893).comments();
    let comments: Vec<Comment> = comments
        .iter(Default::default())
        .try_collect()
        .await
        .unwrap();

    assert!(comments.len() > 0);
}

#[tokio::test]
async fn test_user_followings() {
    let followings = client().user(7466893).followings();
    let users: Vec<User> = followings
        .iter(Default::default())
        .take(50)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(50, users.len());
}

#[tokio::test]
async fn test_user_followers() {
    let followers = client().user(7466893).followers();
    let users: Vec<User> = followers
        .iter(Default::default())
        .take(50)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(50, users.len());
}

#[tokio::test]
async fn test_user_likes() {
    let likes = client().user(7466893).likes();
    let tracks: Vec<Track> = likes
        .iter(Default::default())
        .take(50)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(50, tracks.len());
}

#[tokio::test]
async fn test_track_comments() {
    let comments = client().track(263801976).comments();
    let comments: Vec<Comment> = comments
        .iter(Default::default())
        .take(50)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(50, comments.len());
}

#[tokio::test]
async fn test_track_likers() {
    let likers = client().track(263801976).likers();
    let users: Vec<User> = likers
        .iter(Default::default())
        .take(50)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(50, users.len());
}

#[tokio::test]
async fn test_related_tracks() {
    let related = client().track(263801976).related_tracks();
    let tracks: Vec<Track> = related
        .iter(Default::default())
        .take(30)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(30, tracks.len());
}
