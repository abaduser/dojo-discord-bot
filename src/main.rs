use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context};
use serenity::prelude::*;
use serenity::model::{
    channel::Message,
    gateway::Ready};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{command, group}
};

const NONSENSE: [&str;4] = ["Quake3 net", "unlag", "OSP", "Q3 net"];


#[command]
async fn welcome(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Welcome you filthy animal").await?;
    Ok(())
}
#[group]
#[commands(welcome)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx:Context, msg: Message){
        for n in NONSENSE{
            if msg.content.contains(n){
                msg.channel_id.say(&ctx, "Sorry but I was programmed to explain why Quake3 netcode is shite\nhttps://cdn.discordapp.com/attachments/1039074867816955914/1067954505737568306/halfspeedserverview1.mp4")
                .await.expect("Could not net-code reply.");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready){
        println!("{} has arrived.", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    
    let framework: StandardFramework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let bot_token = match  env::var_os("DOJO_BOT_TOKEN").unwrap().into_string() {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e)
    };
    //assert!(serenity::utils::validate_token(&bot_token).is_ok());
    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES ;

    let mut client = Client::builder(bot_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client.");
    

    if let Err(why) = client.start().await {
        println!("Err {:?}", why);
    }    
}
