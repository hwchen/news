#![feature(async_await)]

use futures_util::TryStreamExt;
use hyper::Client;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;
use tracing::{
    field,
    info,
};

lazy_static! {
    static ref HN_BASE: url::Url = "https://hacker-news.firebaseio.com/v0/".parse().unwrap();
    static ref HN_ITEM: url::Url = HN_BASE.join("item/").unwrap();
    static ref HN_TOP: hyper::Uri = HN_BASE.join("topstories.json").unwrap().as_str().parse().unwrap();
    static ref HN_DISCUSSION: url::Url = "https://news.ycombinator.com/item".parse().unwrap();
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // set up logging
    let subscriber = tracing_fmt::FmtSubscriber::builder()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // instantiate client
    let https = HttpsConnector::new(4)?;
    let client = Client::builder()
        .build::<_, hyper::Body>(https);


    let tops = fetch_hn_top(&client).await?;

    for top in tops.iter().take(30) {
        let item = fetch_hn_item(*top, &client).await?;

        let mut hn_discussion_url = HN_DISCUSSION.clone();
        hn_discussion_url
            .set_query(Some(&format!("id={}", item.id)));
        let hn_discussion_url = hn_discussion_url.as_str();

        let mut line = item.title;

        if let Some(item_url) = item.url {
            line.push_str("\n  ");
            line.push_str(&item_url);
        }

        line.push_str("\n  ");
        line.push_str(hn_discussion_url);

        println!("{}", line);
    }

    Ok(())
}

#[tracing::instrument]
async fn fetch_hn_top<C>(client: &Client<C, hyper::Body>) -> Result<Vec<u32>>
    where C: hyper::client::connect::Connect + 'static
{
    // Why does `get` consume the uri?
    let res = client.get(HN_TOP.clone()).await?;

    info!(
        url = field::debug(&*HN_TOP),
        status = res.status().as_u16(),
        headers = field::debug(res.headers()),
    );

    let bytes = res.into_body().try_concat().await?;
    info!(
        body = std::str::from_utf8(&bytes)?,
    );

    let users: Vec<_> = serde_json::from_slice(&bytes)?;

    Ok(users)
}

#[tracing::instrument]
async fn fetch_hn_item<C>(item: u32, client: &Client<C, hyper::Body>) -> Result<Item>
    where C: hyper::client::connect::Connect + 'static
{
    let url = HN_ITEM
        .join(&format!("{}.json", item))?;

    let res = client.get(url.as_str().parse()?).await?;

    info!(
        url = url.as_str(),
        status = res.status().as_u16(),
        headers = field::debug(res.headers()),
    );

    let bytes = res.into_body().try_concat().await?;

    let item = serde_json::from_slice(&bytes)?;

    Ok(item)
}

#[derive(Debug, Deserialize)]
struct Item {
    id: i32,
    #[serde(rename="type")]
    item_type: String,
    title: String,
    url: Option<String>,
}
