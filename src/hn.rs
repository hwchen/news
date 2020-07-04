use lazy_static::lazy_static;
use serde::Deserialize;
use log::{
    debug,
    info,
};

use crate::Result;

lazy_static! {
    static ref HN_BASE: url::Url = "https://hacker-news.firebaseio.com/v0/".parse().unwrap();
    static ref HN_ITEM: url::Url = HN_BASE.join("item/").unwrap();
    static ref HN_TOP: String = HN_BASE.join("topstories.json").unwrap().as_str().to_owned();
    pub(crate) static ref HN_DISCUSSION: url::Url = "https://news.ycombinator.com/item".parse().unwrap();
}

pub fn fetch_hn_top() -> Result<Vec<u32>>
{
    let res = ureq::get(&HN_TOP).call();

    info!("{url}, {status}",
        url = HN_TOP.to_string(),
        status = res.status(),
    );

    let users: Vec<_> = res.into_json_deserialize()?;

    Ok(users)
}

pub fn fetch_hn_item(item: u32) -> Result<Item>
{
    let url = HN_ITEM
        .join(&format!("{}.json", item))?;

    let res = ureq::get(url.as_str()).call();

    info!("{url}, {status}",
        url = url.as_str(),
        status = res.status(),
    );

    let item = res.into_json_deserialize()?;

    debug!("{:?}", item);

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
