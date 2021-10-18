use std::env;
use std::{thread, time};

use egg_mode::tweet::DraftTweet;
use gptj;

mod db;
use db::{read_db, write_db};

#[tokio::main]
async fn main() -> egg_mode::error::Result<()> {
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

    loop {
        match read_feed_and_tweet(&mut tweets, &token).await {
            Ok(()) => {}
            Err(err) => println!("{:?}", err),
        }
        thread::sleep(time::Duration::from_secs(60));
    }
}

async fn read_feed_and_tweet(
    tweets: &mut Vec<u64>,
    token: &egg_mode::Token,
) -> egg_mode::error::Result<()> {
    let timeline = egg_mode::tweet::home_timeline(&token).with_page_size(10);

    let (_timeline, feed) = timeline.start().await?;
    for tweet in feed.response {
        if !tweets.contains(&tweet.id) {
            println!("");
            let tweet_user = &tweet.user.unwrap();
            println!("{}: {}", tweet_user.screen_name, &tweet.text);

            if tweet_user.screen_name != env::var("USER_NAME").unwrap() {
                match ai_response(tweet.text).await {
                    Ok(response) => {
                        let text: String = format!(
                            "https://twitter.com/{}/status/{} {}",
                            tweet_user.screen_name, tweet.id, response
                        )
                        .chars()
                        .into_iter()
                        .take(240)
                        .collect();
                        match text.len() > 0 {
                            true => {
                                let my_tweet = DraftTweet::new(text.to_string());
                                my_tweet.send(token).await?;
                                tweets.push(tweet.id);
                                println!("ai: {}", text);
                            }
                            false => {}
                        };
                    }
                    Err(err) => {
                        println!("error: {}", err);
                    }
                }
            }

            println!("---");
        }
    }
    write_db(tweets, "tweets.db");
    Ok(())
}

async fn ai_response(context: String) -> Result<String, reqwest::Error> {
    let gpt = gptj::GPT::default();
    Ok(gpt.generate(context, 42, 0.9, 0.9, None).await?.text)
}
