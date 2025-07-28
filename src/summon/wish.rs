#![allow(non_snake_case)]
use crate::summon::{generate, pool::*};
use crate::utils::driver::*;

use std::{fs, path::Path, collections::HashMap};

use poise::serenity_prelude::CreateAttachment;
use rand::Rng;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

static DATA_FILE_PATH: &str = "assets/stored_data/wish_data.json";
static BASE_FIVE_STAR_CHANCE: f64 = 0.006;
static BASE_FOUR_STAR_CHANCE: f64 = 0.051;

#[derive(Serialize, Deserialize, Debug, Default)]
struct UserData {
    totalPulls: usize,
    fiveStarPity: usize,
    fourStarPity: usize,
    fourStarGuarantee: bool,
    fiveStarGuarantee: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct GuildData {
    totalPulls: usize,
    totalFourStars: usize,
    totalWon: usize,
    totalLost: usize,
    totalRateUp: usize,
    totalFiveStars: usize,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct WishData {
    users: HashMap<String, UserData>,
    guilds: HashMap<String, GuildData>,
}

fn load_wish_data() -> WishData {
    if Path::new(DATA_FILE_PATH).exists() {
        let raw_data = fs::read_to_string(DATA_FILE_PATH).expect("Failed to read json file");
        serde_json::from_str(&raw_data).unwrap_or_else(|_| WishData::default())
    } else {
        if let Some(to_create) = Path::new(DATA_FILE_PATH).parent() {
            fs::create_dir_all(to_create).unwrap();
        }
        WishData::default()
    }
}

fn save_wish_data(data: &WishData) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize wish data");
    fs::write(DATA_FILE_PATH, json).expect("Failed to write wish data file");
}

fn summon(user_id: &str, guild_id: &str, pulls: usize) -> Result<Vec<String>, Error> {
    let mut data = load_wish_data();
    let mut rng = rand::rng();
    rng.reseed()?;

    let user_data = data.users.entry(user_id.to_string()).or_default();
    let guild_data = data.guilds.entry(guild_id.to_string()).or_default();

    user_data.totalPulls += pulls;
    guild_data.totalPulls += pulls;

    let mut results: Vec<String> = Vec::new();
    let mut five_star_chance = BASE_FIVE_STAR_CHANCE;

    for _ in 0..pulls {
        let result;
        let mut four_star_flag = false;
        let mut five_star_flag = false;

        if user_data.fiveStarPity >= 75 {
            let extra_chance = (user_data.fiveStarPity - 74) as f64 * 0.06;
            five_star_chance = 1.0_f64.min(five_star_chance + extra_chance);
        }

        if user_data.fiveStarPity >= 89 || rng.random::<f64>() < five_star_chance {
            user_data.fiveStarPity = 0;
            five_star_chance = BASE_FIVE_STAR_CHANCE;
            five_star_flag = true;
            guild_data.totalFiveStars += 1;

            if user_data.fiveStarGuarantee || rng.random::<f64>() < 0.5 {
                result = FIVE_STAR_CHARACTERS_LIMITED
                    .choose(&mut rng)
                    .map(|&pull| pull.to_string())
                    .ok_or("Five star limited character list is empty!")?;
                if !user_data.fiveStarGuarantee {
                    guild_data.totalWon += 1;
                }
                user_data.fiveStarGuarantee = false;
                guild_data.totalRateUp += 1;
            } else {
                result = FIVE_STAR_CHARACTERS_STANDARD
                    .choose(&mut rng)
                    .map(|&pull| pull.to_string())
                    .ok_or("Five star standard character list is empty!")?;
                if !user_data.fiveStarGuarantee {
                    guild_data.totalLost += 1;
                }
                user_data.fiveStarGuarantee = true;
            }
        } else if user_data.fourStarPity >= 9 || rng.random::<f64>() < BASE_FOUR_STAR_CHANCE {
            user_data.fourStarPity = 0;
            guild_data.totalFourStars += 1;
            four_star_flag = true;

            if user_data.fourStarGuarantee || rng.random::<f64>() < 0.5 {
                result = FOUR_STAR_CHARACTERS
                    .choose(&mut rng)
                    .map(|&pull| pull.to_string())
                    .ok_or("Four star character list is empty!")?;
                user_data.fourStarGuarantee = false;
            } else {
                let four_stars: Vec<_> = FOUR_STAR_CHARACTERS
                    .iter()
                    .chain(FOUR_STAR_WEAPONS)
                    .collect();
                result = four_stars
                    .choose(&mut rng)
                    .map(|&&pull| pull.to_string())
                    .ok_or("Four star lists are empty!")?;
                user_data.fourStarGuarantee = true;
            }
        } else {
            result = THREE_STAR_WEAPONS
                .choose(&mut rng)
                .map(|&pull| pull.to_string())
                .ok_or("Four star character list is empty!")?;
        }

        if !five_star_flag {
            user_data.fiveStarPity += 1;
        }
        if !four_star_flag {
            user_data.fourStarPity += 1;
        }

        results.push(result);
    }

    save_wish_data(&data);
    Ok(results)
}

#[poise::command(slash_command, prefix_command)]
pub async fn wish(
    ctx: Context<'_>,
    #[description = "Perform between 1 and 90 Genshin Impact wishes"] text: String,
) -> Result<(), crate::utils::driver::Error> {
    let channel_name = ctx
        .channel_id()
        .name(ctx.serenity_context().http.clone())
        .await?;
    if ctx.author().bot || channel_name != "wishing" {
        ctx.say("You must be in the 'wishing' channel to use this command")
            .await?;
        return Ok(());
    }
    ctx.defer().await?;

    let data = load_wish_data();
    let user_id = ctx.author().id.get().to_string();
    let users = data.users;
    let default_user = UserData::default();
    let user_data = users.get(&user_id).unwrap_or(&default_user);

    let guild_id = ctx.guild_id().ok_or("Expected Guild")?.get().to_string();
    let guilds = data.guilds;
    let default_guild = GuildData::default();
    let guild_data = guilds.get(&guild_id).unwrap_or(&default_guild);

    if text == "me" {
        let total_pulls = user_data.totalPulls;
        let five_star_pity = user_data.fiveStarPity;
        let four_star_pity = user_data.fourStarPity;
        let four_star_guarantee = user_data.fourStarGuarantee.then_some("Yes").unwrap_or("No");
        let five_star_guarantee = user_data.fiveStarGuarantee.then_some("Yes").unwrap_or("No");
        ctx.say(format!(
            r#"**Your Wish Pity Information**:
- **Total Pulls**: {total_pulls}
- **4-Star Pity**: {four_star_pity} pulls since last 4-star
- **5-Star Pity**: {five_star_pity} pulls since last 5-star
- **4-Star Guarantee**: {four_star_guarantee}
- **5-Star Guarantee**: {five_star_guarantee}
"#
        ))
        .await?;
    } else if text == "server" {
        let total_pulls = guild_data.totalPulls;
        let total_four_stars = guild_data.totalFourStars;
        let total_won = guild_data.totalWon;
        let total_lost = guild_data.totalLost;
        let total_rate_up = guild_data.totalRateUp;
        let total_five_stars = guild_data.totalFiveStars;
        ctx.say(format!(
            r#"**Server Wish Data**:
- **Total Pulls**: {total_pulls}
- **Total 4-Star Wishes**: {total_four_stars}
- **Total 50/50\'s Won**: {total_won}
- **Total 50/50\'s Lost**: {total_lost}
- **Total Rate-Up Characters**: {total_rate_up}
- **Total 5-Star Characters**: {total_five_stars}
"#
        ))
        .await?;
    } else {
        let pulls = text.parse::<usize>()?;
        if 1 <= pulls && pulls <= 90 {
            let results = summon(&user_id, &guild_id, pulls)?;
            let (bytes, name) =
                tokio::task::spawn_blocking(move || generate::combine_images(&user_id, results))
                    .await
                    .map_err(|e| Box::new(e) as Error)??;
            ctx.send(
                poise::CreateReply::default().attachment(CreateAttachment::bytes(bytes, name)),
            )
            .await?;
        } else {
            ctx.say("Please input a value between 1 and 90").await?;
        }
    }
    Ok(())
}
