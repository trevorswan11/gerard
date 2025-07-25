use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

static DATA_FILE_PATH: &str = "assets/stored_data/wish_data.json";

struct WishData {
    users: serde_json::Value,
    guilds: serde_json::Value,
}

