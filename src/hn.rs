use futures_util::TryStreamExt;
use hyper::Client;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;
use tracing::{
    debug,
    info,
};

use crate::Result;

lazy_static! {
    static ref HN_BASE: url::Url = "https://hacker-news.firebaseio.com/v0/".parse().unwrap();
    static ref HN_ITEM: url::Url = HN_BASE.join("item/").unwrap();
    static ref HN_TOP: hyper::Uri = HN_BASE.join("topstories.json").unwrap().as_str().parse().unwrap();
    pub(crate) static ref HN_DISCUSSION: url::Url = "https://news.ycombinator.com/item".parse().unwrap();
}

#[tracing::instrument]
pub async fn fetch_hn_top<C>(client: &Client<C, hyper::Body>) -> Result<Vec<u32>>
    where C: hyper::client::connect::Connect + 'static
{
    // Why does `get` consume the uri?
    let res = client.get(HN_TOP.clone()).await?;

    info!(
        url = ?&*HN_TOP,
        status = res.status().as_u16(),
        headers = ?res.headers(),
    );

    let bytes = res.into_body().try_concat().await?;

    debug!(
        body = std::str::from_utf8(&bytes)?,
    );

    let users: Vec<_> = serde_json::from_slice(&bytes)?;

    Ok(users)
}

#[tracing::instrument]
pub async fn fetch_hn_item<C>(item: u32, client: &Client<C, hyper::Body>) -> Result<Item>
    where C: hyper::client::connect::Connect + 'static
{
    let url = HN_ITEM
        .join(&format!("{}.json", item))?;

    let res = client.get(url.as_str().parse()?).await?;

    info!(
        url = url.as_str(),
        status = res.status().as_u16(),
        headers = ?res.headers(),
    );

    let bytes = res.into_body().try_concat().await?;

    let item = serde_json::from_slice(&bytes)?;

    debug!(
        ?item,
    );

    Ok(item)
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: i32,
    #[serde(rename="type")]
    pub item_type: String,
    pub title: String,
    pub url: Option<String>,
}
