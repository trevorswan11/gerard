#![allow(deprecated, dead_code)]
mod external;
mod replies;
mod summon;
mod utils;

use std::env;

use dotenv;
use poise::serenity_prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    utils::driver::start_bot(&token, intents).await;
}
