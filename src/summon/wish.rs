#![allow(non_snake_case)]
use crate::summon::pool::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

static DATA_FILE_PATH: &str = "assets/stored_data/wish_data.json";

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
        WishData::default()
    }
}

fn save_wish_data(data: &WishData) {
    let json = serde_json::to_string_pretty(data)
        .expect("Failed to serialize wish data");
    fs::write(DATA_FILE_PATH, json)
        .expect("Failed to write wish data file");
}

fn summon(user_id: &str, guild_id: &str, pulls: usize) -> Vec<String> {
    let mut data = load_wish_data();

    let user_data = data.users.entry(user_id.to_string()).or_default();
    let guild_data = data.guilds.entry(guild_id.to_string()).or_default();

    user_data.totalPulls += pulls;
    guild_data.totalPulls += pulls;

    let mut result: Vec<String> = Vec::new();
    let mut five_star_chance = 0.006;
    
    for i in 0..pulls {

    }

    save_wish_data(&data);
    result
}