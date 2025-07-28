use crate::utils::driver::*;

use std::{collections::HashSet, env};

use reqwest::Client;
use serde_json::Value;

pub struct Dictionary {
    client: Client,
    dict_key: String,
    thesaurus_key: String,
    avoid_words: HashSet<String>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            dict_key: env::var("DICT_KEY").expect("DICT_KEY missing"),
            thesaurus_key: env::var("THESAURUS_KEY").expect("THESAURUS_KEY missing"),
            avoid_words: HashSet::from(["hard".to_string()]),
        }
    }

    pub async fn definition(&self, word: &str) -> Result<Option<Vec<String>>, Error> {
        let url = format!(
            "https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}",
            word, self.dict_key
        );

        let res: Value = self.client.get(&url).send().await?.json().await?;
        let definition = res
            .get(0)
            .and_then(|d| d.get("shortdef"))
            .and_then(|defs| defs.as_array())
            .map(|defs| {
                defs.iter()
                    .filter_map(|d| d.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });
        Ok(definition)
    }

    pub async fn synonyms(&self, word: &str) -> Result<Option<Vec<String>>, Error> {
        let url = format!(
            "https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{}?key={}",
            word, self.thesaurus_key
        );

        let res: Value = self.client.get(&url).send().await?.json().await?;
        let synonyms = res
            .get(0)
            .and_then(|entry| entry.get("meta"))
            .and_then(|meta| meta.get("syns"))
            .and_then(|syns| syns.as_array())
            .map(|groups| {
                groups
                    .iter()
                    .flat_map(|g| {
                        g.as_array()
                            .map(|inner| {
                                inner
                                    .iter()
                                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                                    .collect::<Vec<String>>()
                            })
                            .unwrap_or_default()
                    })
                    .collect::<Vec<String>>()
            });

        Ok(synonyms)
    }

    pub async fn antonyms(&self, word: &str) -> Result<Option<Vec<String>>, Error> {
        let url = format!(
            "https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{}?key={}",
            word, self.thesaurus_key
        );

        let res: Value = self.client.get(&url).send().await?.json().await?;
        let antonyms = res
            .get(0)
            .and_then(|entry| entry.get("meta"))
            .and_then(|meta| meta.get("ants"))
            .and_then(|ants| ants.as_array())
            .map(|groups| {
                groups
                    .iter()
                    .flat_map(|g| {
                        g.as_array()
                            .map(|inner| {
                                inner
                                    .iter()
                                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                                    .collect::<Vec<String>>()
                            })
                            .unwrap_or_default()
                    })
                    .collect::<Vec<String>>()
            });

        Ok(antonyms)
    }
}

async fn send_response(
    ctx: Context<'_>,
    message: Result<Option<String>, Error>,
    description: &str,
    word: &str,
) -> Result<(), Error> {
    match message {
        Ok(Some(msg)) => ctx.say(msg).await?,
        Ok(None) => {
            ctx.say(format!("No {} found for: {}", description, word))
                .await?
        }
        _ => ctx.say("Error processing command").await?,
    };
    return Ok(());
}

#[poise::command(slash_command, prefix_command)]
pub async fn define(
    ctx: Context<'_>,
    #[description = "Define a word"] word: String,
) -> Result<(), Error> {
    let dict = Dictionary::new();
    let msg = dict
        .definition(&word)
        .await
        .map(|s| s.map(|results| results.join("; ")));
    send_response(ctx, msg, "definitions", &word).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn synonyms(
    ctx: Context<'_>,
    #[description = "Get synonyms of a word"] word: String,
) -> Result<(), Error> {
    let dict = Dictionary::new();
    let msg = dict
        .synonyms(&word)
        .await
        .map(|s| s.map(|results| results.join("; ")));
    send_response(ctx, msg, "synonyms", &word).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn antonyms(
    ctx: Context<'_>,
    #[description = "Get antonyms of a word"] word: String,
) -> Result<(), Error> {
    let dict = Dictionary::new();
    let msg = dict
        .antonyms(&word)
        .await
        .map(|s| s.map(|results| results.join("; ")));
    send_response(ctx, msg, "antonyms", &word).await
}
