use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("+startvote") {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, msg.content.replace("+startvote", "== VOTING =="))
                .await
            {
                println!("Error :( - {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is bitconneeeeected", ready.user.name);
    }
}
