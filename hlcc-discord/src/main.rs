// hlcc-discord  -  hlcc discord bot
// Copyright (C) 2022 Jana Marie Hemsing

use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

fn run_parser(input: &str) -> String {
    let res = hlcc_parser::compute(&input);
    match res {
        Ok(out) => format!("{} computes to\n{}", input, out),
        Err(e) => format!("{} :(", e),
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!convert") {
            let mut msg_chars = msg.content.chars();
            // Remove „!convert" to prevent the bot from calling itself
            let _msg_prefix: String = msg_chars.by_ref()
                .take_while(|&c| c != ' ')
                .collect();
            // run parser and reply with result
            if let Err(why) = msg.channel_id.say(&ctx.http, run_parser(msg_chars.as_str())).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
