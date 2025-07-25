#![allow(non_snake_case)]
use crate::utils::driver::*;

use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use base64::decode;
use hound::{SampleFormat, WavSpec, WavWriter};
use poise::serenity_prelude::{self as serenity};
use serde::{Deserialize, Serialize};
use songbird::input::Input;
use symphonia::core::{
    audio::{SampleBuffer, Signal}, codecs::DecoderOptions, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};
use symphonia::default::{get_codecs, get_probe};
use tokio::time::sleep;
use yup_oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};

static ACCESS_TOKEN_PATH: &str = "service_account.json";

#[derive(Serialize)]
struct TtsRequest<'a> {
    input: TtsInput<'a>,
    voice: TtsVoice<'a>,
    audioConfig: TtsAudioConfig<'a>,
}

#[derive(Serialize)]
struct TtsInput<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct TtsVoice<'a> {
    languageCode: &'a str,
    ssmlGender: &'a str,
    name: &'a str,
}

#[derive(Serialize)]
struct TtsAudioConfig<'a> {
    audioEncoding: &'a str,
}

#[derive(Deserialize)]
struct TtsResponse {
    audioContent: String,
}

async fn load_service_account_key(path: &str) -> Result<ServiceAccountKey, Error> {
    let contents = fs::read_to_string(path)?;
    let key: ServiceAccountKey = serde_json::from_str(&contents)?;
    Ok(key)
}

async fn get_access_token() -> Result<String, Error> {
    let key = load_service_account_key(ACCESS_TOKEN_PATH).await?;
    let auth = ServiceAccountAuthenticator::builder(key).build().await?;

    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let access_token = auth.token(scopes).await?;
    access_token
        .token()
        .map(|s| s.to_string())
        .ok_or("Could not decode valid access token".into())
}

async fn generate_mp3(text: String, output_path: &PathBuf) -> Result<(), Error> {
    let token = get_access_token().await?;

    let request_body = TtsRequest {
        input: TtsInput { text: &text },
        voice: TtsVoice {
            languageCode: "en-US",
            ssmlGender: "MALE",
            name: "en-US-Wavenet-D",
        },
        audioConfig: TtsAudioConfig {
            audioEncoding: "MP3",
        },
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://texttospeech.googleapis.com/v1/text:synthesize")
        .bearer_auth(token)
        .json(&request_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Google TTS API error: {}", text).into());
    }
    let tts_response: TtsResponse = res.json().await?;

    let audio_data = match decode(&tts_response.audioContent) {
        Ok(d) => d,
        Err(e) => {
            return Err(format!("Failed to decode audio content: {}", e).into());
        }
    };

    tokio::fs::create_dir_all(output_path.parent().unwrap()).await?;
    let mut file = File::create(&output_path)?;
    file.write_all(&audio_data)?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn tts(
    ctx: Context<'_>,
    #[description = "Convert text to speech"] text: String,
) -> Result<(), crate::utils::driver::Error> {
    let guild = ctx.guild_id().ok_or("Could not get guild ID")?;

    let voice_channel_id = match ctx.guild_channel().await {
        Some(guild_channel) => {
            let channels = guild.channels(ctx.serenity_context()).await?;
            let channel = channels
                .get(&guild_channel.id)
                .ok_or("Did not recognize channel id")?;
            if channel.kind == serenity::ChannelType::Voice {
                guild_channel.id
            } else {
                ctx.say("You need to be in a voice channel to use this command!")
                    .await?;
                return Ok(());
            }
        }
        None => {
            ctx.say("Did not recognize channel id").await?;
            return Ok(());
        }
    };

    let output_path = PathBuf::from(format!(
        "./assets/gen/{}-tts.mp3",
        voice_channel_id.to_string()
    ));
    generate_mp3(text, &output_path).await?;
    ctx.say("Playing your message...").await?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let join_result = manager.join(guild, voice_channel_id).await;
    if let Ok(handler_lock) = join_result {
        let mut handler = handler_lock.lock().await;
        let path_str = output_path
            .as_path()
            .to_str()
            .ok_or_else(|| "Invalid path buffer".to_string())?
            .to_string();
        let source = Input::try_from(path_str)?;
        let track_handle = handler.play_input(source);
        while let Some(state) = track_handle.get_info().await.ok() {
            if state.playing.is_done() {
                break;
            }
            sleep(Duration::from_millis(250)).await;
        }

        handler.leave().await?;
    } else {
        ctx.say("Failed to join voice channel.").await?;
    }
    Ok(())
}
