use dotenv::dotenv;
use std::env;
use std::{thread, time};

use egg_mode::tweet::DraftTweet;

mod db;
use db::{read_db, write_db};

mod ai;
use ai::response;

#[tokio::main]
async fn main() -> egg_mode::error::Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();
    let api_secret = env::var("API_KEY_SECRET").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap_or_default();
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET").unwrap_or_default();

    let con_token = egg_mode::KeyPair::new(api_key, api_secret);

    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let mut tweets: Vec<u64> = read_db("tweets.db");
    let ai = ai::create_ai(env::var("AI").unwrap_or_default());
    println!("Using ai: {}", ai.name());
    loop {
        match read_feed_and_tweet(&ai, &mut tweets, &token).await {
            Ok(()) => {}
            Err(err) => println!("{:?}", err),
        }
        thread::sleep(time::Duration::from_secs(60));
    }
}

async fn read_feed_and_tweet(
    ai: &Box<dyn ai::AI>,
    tweets: &mut Vec<u64>,
    token: &egg_mode::Token,
) -> egg_mode::error::Result<()> {
    let timeline = egg_mode::tweet::home_timeline(token).with_page_size(10);

    let (_timeline, feed) = timeline.start().await?;
    for tweet in feed.response {
        if !tweets.contains(&tweet.id) {
            println!();
            let tweet_user = &tweet.user.unwrap();
            println!("{}: {}", tweet_user.screen_name, &tweet.text);

            if tweet_user.screen_name != env::var("USER_NAME").unwrap() {
                let url = format!(
                    "https://twitter.com/{}/status/{}",
                    tweet_user.screen_name, tweet.id
                );
                let response = response(ai, tweet.text, url, 240).await;
                match !response.is_empty() {
                    true => {
                        let my_tweet = DraftTweet::new(response.to_string());
                        my_tweet.send(token).await?;
                        tweets.push(tweet.id);
                        println!("ai: {}", response);
                    }
                    false => {}
                };
            }

            println!("---");
        }
    }
    write_db(tweets, "tweets.db");
    Ok(())
}
