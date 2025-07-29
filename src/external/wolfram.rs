use crate::utils::driver::*;

use std::env;

use reqwest::Client;
use urlencoding::encode;

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Ask WolframAlpha to answer a query")
)]
pub async fn wolfram(
    ctx: Context<'_>,
    #[description = "Your query"] query: String,
) -> Result<(), Error> {
    let app_id = env::var("WOLFRAM_SIMPLE").expect("WOLFRAM_SIMPLE missing");
    ctx.defer().await?;

    let encoded_query = encode(&query);
    let url = format!(
        "http://api.wolframalpha.com/v1/result?appid={}&i={}",
        app_id, encoded_query
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let text = response.text().await?;
        ctx.say(text).await?;
    } else {
        ctx.say("Wolfram|Alpha could not understand your input or no short results were found.")
            .await?;
    }
    Ok(())
}
