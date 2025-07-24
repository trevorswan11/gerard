use std::{error::Error, fs, fs::File, io::Write, path::PathBuf, sync::Arc, time::Duration};

use base64::decode;
use poise::serenity_prelude as serenity;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use songbird::{
    Event, EventContext, EventHandler as VoiceEventHandler, SerenityInit, input::Input,
};
use tokio::time::sleep;
use yup_oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};

pub static ACCESS_TOKEN_PATH: &str = "service_account.json";

#[derive(Serialize)]
struct TtsRequest<'a> {
    input: TtsInput<'a>,
    voice: TtsVoice<'a>,
    audio_config: TtsAudioConfig<'a>,
}

#[derive(Serialize)]
struct TtsInput<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct TtsVoice<'a> {
    language_code: &'a str,
    ssml_gender: &'a str,
    name: &'a str,
}

#[derive(Serialize)]
struct TtsAudioConfig<'a> {
    audio_encoding: &'a str,
}

#[derive(Deserialize)]
struct TtsResponse {
    audio_content: String,
}

async fn load_service_account_key(path: &str) -> Result<ServiceAccountKey, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let key: ServiceAccountKey = serde_json::from_str(&contents)?;
    Ok(key)
}

pub async fn get_access_token() -> Result<String, Box<dyn Error>> {
    let key = load_service_account_key(ACCESS_TOKEN_PATH).await?;
    let auth = ServiceAccountAuthenticator::builder(key)
        .build()
        .await?;

    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let access_token = auth.token(scopes).await?;
    if let Some(token_str) = access_token.token() {
        Ok(token_str.to_string())
    } else {
        Err("Could not decode valid access token".into())
    }
}
