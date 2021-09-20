use dotenv::dotenv;
use serenity::framework::standard::CommandResult;
use serenity::prelude::TypeMapKey;
use std::env;
use std::default::Default;

use serenity::client::{Client, Context};
use serenity::framework::standard::{
    macros::{command, group},
    Args, StandardFramework,
};
use serenity::model::channel::Message;

// TODO: move this into vote_data.rs
struct CurrentVote {
    pub title: String,
    pub description: String,
    running: bool,
}

impl Default for CurrentVote {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentVote {
    pub fn new() -> Self {
        CurrentVote {
            title: String::new(),
            description: String::new(),
            running: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl TypeMapKey for CurrentVote {
    type Value = CurrentVote;
}

#[group]
#[commands(start_vote)]
struct General;

// TODO: move this into admin_commands.rs
#[command]
async fn start_vote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;

    // first, check if there is already a vote taking place
    if !data.contains_key::<CurrentVote>() { msg.reply(ctx, "No instance of CurrentVote was found inside of the client data").await; return Ok(())}
    let vote = data.get::<CurrentVote>().unwrap();
    if !vote.is_running() { msg.reply(ctx, "There is a vote already taking place").await; return Ok(())}

    // create a new vote with the given title and description
    let mut new_vote: CurrentVote = Default::default();
    
    new_vote.title = args.single::<String>().unwrap_or(String::new());
    new_vote.description = args.single::<String>().unwrap_or(String::new());

    {
        let mut data = ctx.data.write().await;
        data.insert::<CurrentVote>(new_vote);
    }

    msg.channel_id.say(ctx, "Created a new Vote").await;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = dotenv() {
        panic!("Error loading the .env file: {:?}", error);
    };

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // create the framework and the client
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("#"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
