use std::env;
use serenity::async_trait;
use serenity::client::{Client, Context};
use serenity::framework::standard::help_commands::Command;
use serenity::model::mention;
use serenity::prelude::*;
use serenity::model::{
    mention::Mention,
    guild::Member,
    guild::Role,
    channel::Message,
    gateway::Ready};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{command, group}
};

const NONSENSE: [&str;8] = ["quake3 net", "quake 3 net", "q 3 net", "q3 net", "unlag", "osp net", "cpma unlag", "cpma netcode"];
const Q3_REPLY_STRING: &str = "Quake3 is shit\nhttps://cdn.discordapp.com/attachments/1039074867816955914/1067954505737568306/halfspeedserverview1.mp4\nPlay QuakeLive for the most up to date netcode";

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn guild_member_addition(&self, ctx:Context, member:Member){
        let info = format!("Hello {}, welcome to The Dojo. We approve new members manually, so while you wait check out https://thedojo.ninja/ for information about us and our servers.", member.display_name());
        match member.user.direct_message(ctx, |m| m.content(info)).await{
            Err(e) => println!("Err sending dm!\n{:?}",e),
            _ => ()
        }
    }

    async fn message(&self, ctx:Context, msg: Message){
        
        match env::var_os("FUCKQ3"){
            Some(s) => {
                if s.into_string().unwrap() == "YES"{
                    msg.reply_ping(ctx, Q3_REPLY_STRING)
                        .await
                        .expect("Err replying");
                        return;
                }
            },
            _ => ()
        }
        for n in NONSENSE{
            if msg.content.to_lowercase().contains(n) && !msg.author.bot {
                msg.reply_ping(&ctx, Q3_REPLY_STRING)
                .await
                .expect("Err replying");
                return;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready){
        println!("{} has arrived.", ready.user.name);
    }
}
#[group]
#[commands(lfd)]
struct General;

#[tokio::main]
async fn main() {
    
    let framework: StandardFramework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let bot_token = env::var_os("DOJO_BOT_TOKEN")
        .expect("No Token set")
        .into_string()
        .unwrap();

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


#[command]
#[description = "Pings people around your rating for duel"]
#[aliases("lookingforduel", "duel")]
async fn lfd(ctx: &Context, msg : &Message) -> CommandResult{
    let m = msg.member(ctx).await?;
    let member_roles: Vec<Role> = m.roles(ctx).unwrap();
    let mut highest_position = 0i64;
    for role in member_roles{
        if role.position > highest_position { highest_position = role.position; }
    }
    let mut roles_to_ping: String = String::from("**Looking for Duel**");
    for guild_role in msg.guild(ctx).unwrap().roles.values() {
        if guild_role.position <= highest_position && guild_role.position > 0 || guild_role.position == highest_position + 1 {
            roles_to_ping.push_str(&format!(" {} ", Mention::from(guild_role.id)));
        }
    }
    msg.channel_id.send_message(ctx, |m| m.content(roles_to_ping)).await?;

    return Ok(())
}
