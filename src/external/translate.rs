#![allow(non_snake_case)]
use crate::utils::driver::*;
use crate::external::languages::LanguageCode;

use std::{env, collections::HashSet};

use urlencoding::encode;
use reqwest::Client;
use serde::{Deserialize};

#[derive(Deserialize)]
struct TranslateResponse {
    data: TranslationData,
}

#[derive(Deserialize)]
struct TranslationData {
    translations: Vec<TranslatedText>,
}

#[derive(Deserialize)]
struct TranslatedText {
    translatedText: String,
}

#[derive(Deserialize)]
struct LanguageResponse {
    data: LanguageData,
}

#[derive(Deserialize)]
struct LanguageData {
    languages: HashSet<LanguageOpt>,
}

#[derive(Deserialize, Hash, PartialEq, Eq)]
struct LanguageOpt {
    language: String,
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Translate text to a desired language")
)]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "The language code you wish to translate to"]
    code: LanguageCode,
    #[rest]
    #[description = "The text you wish to translate"]
    query: String,
) -> Result<(), Error> {
    let api_key = env::var("GOOGLE_KEY").expect("GOOGLE_KEY missing");

    let base = "https://translation.googleapis.com/language/translate/v2";
    let url = format!(
        "{base}?q={}&target={}&format=text&key={}",
        encode(&query),
        encode(code.as_str()),
        encode(&api_key)
    );
    
    let res = Client::new().get(&url).send().await?;
    if res.status().is_success() {
        let json: TranslateResponse = res.json().await.map_err(|e| format!("Parsing error: {}", e))?;
        let translated = &json.data.translations[0].translatedText;
        ctx.say(translated).await?;
    } else {
        ctx.say("Google Translate could not understand your input.")
            .await?;
    }
    Ok(())
}