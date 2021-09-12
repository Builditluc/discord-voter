use std::env;
use dotenv::dotenv;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

#[tokio::main]
async fn main() {
    dotenv();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
