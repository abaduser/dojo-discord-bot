use std::env;

use serenity::client::{Client, Context};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::{StandardFramework,
        CommandResult,
        macros::{command, group}
};



#[group]
struct General;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "hello" {

            ctx.idle().await;
        }
    }
}

#[tokio::main]
async fn main() {
    
    let framework: StandardFramework = StandardFramework::new()
        .configure(|c| c.prefix("!"));
    let bot_token = match  env::var_os("DOJO_BOT_TOKEN").unwrap().into_string() {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e)
    };
    assert!(serenity::utils::validate_token(&bot_token).is_ok());
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(bot_token, intents);
    

    if let Err(why) = client.start().await {
        println!("Err {:?}", why);
    }

    
}
