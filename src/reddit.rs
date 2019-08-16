use futures_util::TryStreamExt;
use hyper::{Client, Request};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;
use tracing::info;

use crate::Result;

lazy_static! {
}

const MY_USER_AGENT: &str = "linux:reddit-focus:v0.0.1 (by /u/hwchen)";

#[tracing::instrument]
pub async fn fetch_reddit_new<C>(
    client: &Client<C, hyper::Body>,
    subreddit: &str,
    n: usize,
) -> Result<Vec<PostData>>
    where C: hyper::client::connect::Connect + 'static
{
    // set up url
    let url = format!("https://oauth.reddit.com/r/{}/new.json", subreddit);
    let mut url: url::Url = url.parse()?;
    url.set_query(Some(&format!("limit={}", n)));
    let url = url.as_str();

    let req = Request::get(url)
        .header("User-Agent", MY_USER_AGENT)
        .header("Authorization", format!("Bearer {}", "")) // don't know why this works
        .body(hyper::Body::empty())?;

    info!(
        start = "start reddit-timer",
    );
    let res = client.request(req).await?;
    info!(
        end = "end reddit-timer",
    );

    info!(
        url = url,
        status = res.status().as_u16(),
        headers = ?res.headers(),
    );

    let bytes = res.into_body().try_concat().await?;

    info!(
        body = std::str::from_utf8(&bytes)?,
    );

    let new: RedditNew = serde_json::from_slice(&bytes)?;
    let posts = new.data.children.into_iter().map(|post| post.data).collect();

    Ok(posts)
}

#[derive(Debug, Deserialize)]
struct RedditNew {
    pub kind: String,
    pub data: RedditNewData,
}

#[derive(Debug, Deserialize)]
struct RedditNewData {
    pub after: String,
    pub children: Vec<Post>,
}

#[derive(Debug, Deserialize)]
struct Post {
    pub kind: String,
    pub data: PostData,
}

#[derive(Debug, Deserialize)]
pub struct PostData {
    pub id: String,
    pub title: String,
    pub name: String,
    pub author: String,
    pub subreddit_id: String,
    pub subreddit: String,
    pub subreddit_name_prefixed: String,
    pub selftext: String,
    pub permalink: String,
    pub domain: String,
    pub url: String,
}
