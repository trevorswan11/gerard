use std::{fs::File, io::Write, path::PathBuf, time::Duration};

use base64::decode;
use poise::serenity_prelude as serenity;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use songbird::{
    Event, EventContext, EventHandler as VoiceEventHandler, SerenityInit, input::Input,
};
use tokio::time::sleep;

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
