use crate::internal::basic::*;
use crate::external::tts::tts;

use poise::serenity_prelude as serenity;
use songbird::SerenityInit;

pub struct Handler;
pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn start_bot(discord_token: &str, intents: serenity::GatewayIntents) {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), tts()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("Logged in as {}", ready.user.name);
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating Client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
