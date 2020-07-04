mod hn;
mod reddit;

use colored::*;

use crate::hn::{
    fetch_hn_top,
    fetch_hn_item,
    HN_DISCUSSION,
};

use crate::reddit::fetch_reddit_new;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    // set up logging
    env_logger::init();

    // reddit
    let subreddit = "rust";
    let n = 30;
    let new_posts = fetch_reddit_new(subreddit, n)?;

    for post in new_posts.iter() {
        println!("{}", post.title.red());
        if post.domain != "self.rust" {
            println!("  {}", post.url);
        }
        println!("  https://reddit.com{}", post.permalink);
    }

    println!();

    //hn

    // use connection pool
    let agent = ureq::agent();

    let tops = fetch_hn_top(&agent)?;

    for top in tops.iter().take(30) {
        let item = fetch_hn_item(&agent, *top)?;

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

