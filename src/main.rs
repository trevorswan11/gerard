#![allow(deprecated, dead_code)]
mod external;
mod internal;
mod summon;
mod utils;

use std::env;

use dotenvy::dotenv;
use poise::serenity_prelude::GatewayIntents;
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if true {
        let dict = external::dict::Dictionary::new();
        let word = "quick";
        match dict.definition(word).await {
            Ok(Some(defs)) => println!("Definitions for {}: {}", word, defs.join("; ")),
            Ok(None) => println!("No definition found."),
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    utils::driver::start_bot(&token, intents).await;
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
}
