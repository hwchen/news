mod hn;
mod reddit;

use colored::*;
use hyper::Client;
use hyper_tls::HttpsConnector;

use crate::hn::{
    fetch_hn_top,
    fetch_hn_item,
    HN_DISCUSSION,
};

use crate::reddit::fetch_reddit_new;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // set up logging
    tracing_subscriber::fmt::init();

    // instantiate client
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    // reddit
    let subreddit = "rust";
    let n = 30;
    let new_posts = fetch_reddit_new(&client, subreddit, n).await?;

    for post in new_posts.iter() {
        println!("{}", post.title.red());
        if post.domain != "self.rust" {
            println!("  {}", post.url);
        }
        println!("  https://reddit.com{}", post.permalink);
    }

    println!();

    //hn
    let tops = fetch_hn_top(&client).await?;

    for top in tops.iter().take(30) {
        let item = fetch_hn_item(*top, &client).await?;

        let mut hn_discussion_url = HN_DISCUSSION.clone();
        hn_discussion_url
            .set_query(Some(&format!("id={}", item.id)));
        let hn_discussion_url = hn_discussion_url.as_str();

        let mut line = item.title.green().to_string();

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

