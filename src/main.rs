use egg_mode::error::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let con_key = env::var("KEY").unwrap();
    let con_secret = env::var("SECRET").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap_or_default();
    let access_token_secret = env::var("ACCESS_SECRET").unwrap_or_default();

    let con_token = egg_mode::KeyPair::new(con_key, con_secret);

    let access_token = egg_mode::KeyPair::new(
        access_token,
        access_token_secret
    );
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let timeline =
        egg_mode::tweet::home_timeline(&token).with_page_size(5);

    let (_timeline, feed) = timeline.start().await?;
    for tweet in feed.response {
        println!("");
        println!("{}: {}", &tweet.user.unwrap().screen_name, &tweet.text);
    }
    Ok(())
}