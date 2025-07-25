use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

static DATA_FILE_PATH: &str = "assets/stored_data/wish_data.json";

#[derive(Deserialize, Serialize, Default, Debug)]
struct WishData {
    users: serde_json::Value,
    guilds: serde_json::Value,
}

fn load_wish_data() -> WishData {
    if Path::new(DATA_FILE_PATH).exists() {
        let raw_data = fs::read_to_string(DATA_FILE_PATH).expect("Failed to read json file");
        serde_json::from_str(&raw_data).unwrap_or_else(|_| WishData::default())
    } else {
        WishData::default()
    }
}
