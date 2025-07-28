#![allow(deprecated, dead_code)]
mod external;
mod internal;
mod summon;
mod utils;

use std::env;

use dotenv;
use poise::serenity_prelude::GatewayIntents;
use tokio::signal;

fn init_tracing() {
    console_subscriber::init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    dotenv::dotenv().expect("Failed to load .env file");
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
