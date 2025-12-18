use crate::external::{dict::*, qrcode::qrcode, translate::translate, tts::tts, wolfram::wolfram};
use crate::internal::{help::*, jar::jar, locate::locate, replies::*};
use crate::summon::wish::wish;
use crate::utils::lus::*;

use poise::serenity_prelude as serenity;
use serenity::{EventHandler, async_trait, model::channel::Message};
use songbird::SerenityInit;

pub struct Handler;
pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn start_bot(discord_token: &str, intents: serenity::GatewayIntents) {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(),
                tts(),
                wish(),
                define(),
                synonyms(),
                antonyms(),
                wolfram(),
                jar(),
                qrcode(),
                translate(),
                locate(),
                cointoss(),
                plug(),
                bloom(),
                server(),
                lethal(),
                help(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
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
        .event_handler(Handler)
        .await
        .expect("Error creating Client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: Message) {
        if msg.author.bot || msg.content.is_empty() {
            return;
        }
        let lower_msg = msg.content.to_lowercase().to_string();

        // Image 'command' handling
        if let Some(&(name, bytes)) = misc_image_lu().get(lower_msg.as_str()) {
            send_img_delete_msg(&ctx, name, bytes, &msg).await;
        } else if let Some(&link) = gif_lu().get(lower_msg.as_str()) {
            send_gif_delete_msg(&ctx, link, &msg).await;
        } else if let Some(key) = find(COPYPASTA_KEYS, &lower_msg) {
            send_copypasta(&ctx, key, &msg).await;
        } else if let Some(word) = find(NAUGHTY_KEYS, &lower_msg) {
            if let Err(e) = msg.channel_id.say(ctx.http, format!("{word}?")).await {
                eprintln!("Failed to send message: {}", e);
            }
        } else if lower_msg.contains("thank you gerard") {
            if let Err(e) = msg
                .channel_id
                .say(ctx.http, format!("Anytime, {}!", msg.author))
                .await
            {
                eprintln!("Failed to send message: {}", e);
            }
        }
    }
}
