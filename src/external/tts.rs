use crate::utils::driver::*;

use std::{any::Any, fs::{self, File}, io::Write, path::PathBuf, sync::Arc, time::Duration};

use base64::decode;
use poise::serenity_prelude::{self as serenity, futures::channel};
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

async fn load_service_account_key(path: &str) -> Result<ServiceAccountKey, Error> {
    let contents = fs::read_to_string(path)?;
    let key: ServiceAccountKey = serde_json::from_str(&contents)?;
    Ok(key)
}

pub async fn get_access_token() -> Result<String, Error> {
    let key = load_service_account_key(ACCESS_TOKEN_PATH).await?;
    let auth = ServiceAccountAuthenticator::builder(key)
        .build()
        .await?;

    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let access_token = auth.token(scopes).await?;
    access_token
        .token()
        .map(|s| s.to_string())
        .ok_or("Could not decode valid access token".into())
}

#[poise::command(slash_command, prefix_command)]
pub async fn tts_command(
    ctx: Context<'_>,
    #[description = "Convert text to speech"] text: String,
) -> Result<(), crate::utils::driver::Error> {
    let guild = ctx.guild()
        .ok_or("Could not get guild ID")?;
    let member = guild.member(ctx.serenity_context(), ctx.author().id)
        .await?;
    
    let voice_channel_id = match ctx.guild_channel().await {
        Some(guild_channel) => {
            let is_voice = guild.channels(ctx.serenity_context())
                .await?
                .get(&guild_channel.id)
                .ok_or(ctx.say("Did not recognize channel id").await?)
                .unwrap()
                .kind
                .eq(&serenity::ChannelType::Voice);
            if is_voice {
                guild_channel
            } else {
                ctx.say("You need to be in a voice channel to use this command!").await?;
                return Ok(());
            }
        }
        None => {ctx.say("Did not recognize channel id"); return Ok(())}
    };
    Ok(())
}