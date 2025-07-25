#![allow(non_snake_case)]
use crate::utils::driver::*;

use std::{
    fs::{self, File as StdFile},
    io::Write,
    path::PathBuf,
    time::Duration,
};

use base64::decode;
use serde::{Deserialize, Serialize};
use songbird::input::File as SongFile;
use songbird::tracks::Track;
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
    let mut file = StdFile::create(&output_path)?;
    file.write_all(&audio_data)?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn tts(
    ctx: Context<'_>,
    #[description = "Convert text to speech"] text: String,
) -> Result<(), crate::utils::driver::Error> {
    let guild_id = ctx.guild_id().ok_or("Guild ID not available")?;
    let voice_channel_id = {
        let guild = ctx
            .serenity_context()
            .cache
            .guild(guild_id)
            .ok_or_else(|| "Failed to get Guild ID")?;

        let vs = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id);
        vs.ok_or("You're not in a voice channel")?
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
    let source = SongFile::new(output_path);
    let track = Track::from(source).volume(1.0);

    if let Ok(call_lock) = manager.join(guild_id, voice_channel_id).await {
        let mut call = call_lock.lock().await;
        let track_handle = call.play(track);
        while let Some(state) = track_handle.get_info().await.ok() {
            if state.playing.is_done() {
                break;
            }
            sleep(Duration::from_millis(250)).await;
        }

        call.leave().await?;
    } else {
        ctx.say("Failed to join voice channel.").await?;
    }

    Ok(())
}
