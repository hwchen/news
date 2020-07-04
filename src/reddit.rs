use serde::Deserialize;
use log::info;

use crate::Result;

const MY_USER_AGENT: &str = "linux:reddit-focus:v0.0.1 (by /u/hwchen)";

pub fn fetch_reddit_new(
    subreddit: &str,
    n: usize,
) -> Result<Vec<PostData>>
{
    // set up url
    let url = format!("https://oauth.reddit.com/r/{}/new.json", subreddit);
    let mut url: url::Url = url.parse()?;
    url.set_query(Some(&format!("limit={}", n)));
    let url = url.as_str();

    info!("start reddit-timer");

    let res = ureq::get(url)
        .set("User-Agent", MY_USER_AGENT)
        .set("Authorization", &format!("Bearer {}", "")) // don't know why this works
        .call();


    info!("end reddit-timer");

    info!("{url}, {status}",
        url = url,
        status = res.status(),
    );

    let new: RedditNew = res.into_json_deserialize()?;
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
