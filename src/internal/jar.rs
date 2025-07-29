use crate::utils::driver::*;

use std::{collections::HashMap, fs, path::Path};

use chrono::Local;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    quote: String,
    date: String,
}

type UserQuotes = Vec<Quote>;
type GuildQuotes = HashMap<String, UserQuotes>;
type AllQuotes = HashMap<String, GuildQuotes>;

const QUOTE_FILE_PATH: &str = "assets/stored_data/quotes.json";

fn load_quotes() -> AllQuotes {
    if Path::new(QUOTE_FILE_PATH).exists() {
        let raw_quotes = fs::read_to_string(QUOTE_FILE_PATH).expect("Failed to read json file");
        serde_json::from_str(&raw_quotes).unwrap_or_default()
    } else {
        if let Some(to_create) = Path::new(QUOTE_FILE_PATH).parent() {
            fs::create_dir_all(to_create).unwrap();
        }
        HashMap::new()
    }
}

fn save_quotes(quotes: &AllQuotes) {
    let json = serde_json::to_string_pretty(quotes).expect("Failed to serialize quote data");
    fs::write(QUOTE_FILE_PATH, json).expect("Failed to write quote data file");
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Save a quote for a user")
)]
async fn save(
    ctx: Context<'_>,
    #[description = "User to save the quote for, or yourself by default"] user: Option<
        serenity::User,
    >,
    #[rest]
    #[description = "The quote text"]
    quote: String,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let guild_id = ctx
        .guild_id()
        .ok_or("This command must be used in a server")?;
    let mut all_quotes = load_quotes();
    let guild_entry = all_quotes.entry(guild_id.get().to_string()).or_default();
    let user_quotes = guild_entry.entry(user.id.to_string()).or_default();

    user_quotes.push(Quote {
        quote,
        date: Local::now().format("%m/%d/%Y").to_string(),
    });

    save_quotes(&all_quotes);

    ctx.say(format!("Saved quote for {}.", user.name)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "List all quotes for a user")
)]
async fn list(
    ctx: Context<'_>,
    #[description = "User to list quotes for, or yourself by default"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let guild_id = ctx
        .guild_id()
        .ok_or("This command must be used in a server")?;
    let all_quotes = load_quotes();
    let guild_entry = all_quotes.get(&guild_id.get().to_string());

    if let Some(user_quotes) = guild_entry.and_then(|g| g.get(&user.id.to_string())) {
        if user_quotes.is_empty() {
            ctx.say(format!("No quotes found for {}.", user.name))
                .await?;
        } else {
            let mut response = String::from(format!("Quotes for {}:\n", user.name));
            for (i, q) in user_quotes.iter().enumerate() {
                response.push_str(&format!("{}. \"{}\" - {}\n", i + 1, q.quote, q.date));
            }
            ctx.say(response).await?;
        }
    } else {
        ctx.say(format!("No quotes found for {}.", user.name))
            .await?;
    }
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("save", "list"),
    category = "Quotes",
    description_localized("en-US", "Manage or view user quotes")
)]
pub async fn jar(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
