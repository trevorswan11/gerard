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
}
