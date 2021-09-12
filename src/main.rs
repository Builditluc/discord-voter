use dotenv::dotenv;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
mod eventhandler;

#[tokio::main]
async fn main() {
    dotenv();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(eventhandler::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
