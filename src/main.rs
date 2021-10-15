use std::env;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::{thread, time};

use egg_mode::error::Result;
use egg_mode::tweet::DraftTweet;
use gptj;

fn read_db(filename: impl AsRef<Path>) -> Vec<u64> {
    match File::open(filename) {
        Ok(file) => {
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line").parse::<u64>().unwrap())
                .collect()
        }
        Err(_) => vec![],
    }
}

fn write_db(tweets: &Vec<u64>, filename: impl AsRef<Path>) {
    let mut f = File::create(filename).expect("Unable to create file");
    for i in tweets {
        let id = format!("{}\n", i.to_string());
        f.write_all(id.as_bytes()).expect("Unable to write data");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
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

async fn read_feed_and_tweet(tweets: &mut Vec<u64>, token: &egg_mode::Token) -> Result<()> {
    let timeline = egg_mode::tweet::home_timeline(&token).with_page_size(10);

    let (_timeline, feed) = timeline.start().await?;
    for tweet in feed.response {
        if !tweets.contains(&tweet.id) {
            println!("");
            let tweet_user = &tweet.user.unwrap();
            println!("{}: {}", tweet_user.screen_name, &tweet.text);

            if tweet_user.screen_name != env::var("USER_NAME").unwrap() {
                let text: String = format!(
                    "https://twitter.com/{}/status/{} {}",
                    tweet_user.screen_name,
                    tweet.id,
                    ai_response(tweet.text).await
                )
                .chars()
                .into_iter()
                .take(240)
                .collect();
                match text.len() > 0 {
                    true => {
                        tweets.push(tweet.id);
                        let tweet = DraftTweet::new(text.to_string());
                        tweet.send(token).await?;

                        println!("ai: {}", text);
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

async fn ai_response(context: String) -> String {
    let gpt = gptj::GPT::default();
    match gpt.generate(context, 42, 0.9, 0.9, None).await {
        Ok(response) => response.text,
        Err(err) => {
            println!("{}", err);
            "".to_string()
        }
    }
}
