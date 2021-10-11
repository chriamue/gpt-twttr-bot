use egg_mode::error::Result;
use gptj;
use std::env;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

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

fn write_db(tweets: Vec<u64>, filename: impl AsRef<Path>) {
    let mut f = File::create(filename).expect("Unable to create file");
    for i in &tweets {
        let id = format!("{}\n", i.to_string());
        f.write_all(id.as_bytes()).expect("Unable to write data");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let con_key = env::var("KEY").unwrap();
    let con_secret = env::var("SECRET").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap_or_default();
    let access_token_secret = env::var("ACCESS_SECRET").unwrap_or_default();

    let con_token = egg_mode::KeyPair::new(con_key, con_secret);

    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let mut tweets: Vec<u64> = read_db("tweets.db");

    let timeline = egg_mode::tweet::home_timeline(&token).with_page_size(10);

    let (_timeline, feed) = timeline.start().await?;
    for tweet in feed.response {
        if !tweets.contains(&tweet.id) {
            println!("");
            println!("{}: {}", &tweet.user.unwrap().screen_name, &tweet.text);
            println!("ai: {}", ai_tweet(tweet.text.to_string()).await);
            println!("---");
            tweets.push(tweet.id);
        }
    }
    write_db(tweets, "tweets.db");
    Ok(())
}

async fn ai_tweet(context: String) -> String {
    let gpt = gptj::GPT::default();
    let response = gpt.generate(context, 42, 0.9, 0.9, None).await.unwrap();
    response.text
}
