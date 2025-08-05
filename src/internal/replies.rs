use crate::utils::lus::*;
use crate::utils::{chunk_str, driver};

use poise::serenity_prelude::{self as serenity, CreateAttachment, CreateMessage};
use rand::Rng;
use serenity::model::channel::Message;

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Get the age of you or a specified user in this server")
)]
pub async fn age(
    ctx: driver::Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), driver::Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Flip a fair, two-sided coin")
)]
pub async fn cointoss(ctx: driver::Context<'_>) -> Result<(), driver::Error> {
    let result = {
        let mut rng = rand::rng();
        if rng.gen_bool(0.5) { "Heads" } else { "Tails" }
    };

    ctx.say(result).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Github plug")
)]
pub async fn plug(ctx: driver::Context<'_>) -> Result<(), driver::Error> {
    ctx.say("https://github.com/trevorswan11").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Oh my god?!")
)]
pub async fn bloom(ctx: driver::Context<'_>) -> Result<(), driver::Error> {
    ctx.say("OH MY GOD IM BLOOMING :weary: :weary: :weary:")
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Gerard is a peak glazer")
)]
pub async fn glaze(
    ctx: driver::Context<'_>,
    #[description = "Something"] something: String,
) -> Result<(), driver::Error> {
    let glaze_txt = format!(
        "What is {}? 🤔 For the blind, 🦇 it is light.💡\n\
        For the hungry 😣 , it is bread. 🍞 For the sick, 💉 it is the cure. ⚕️\n\
        For the sad 😢 , it is joy. 😂 For the poor 🚫💰 , it is wealth; 💰\n\
        For the debtor ➖ 💰 , it is forgiveness. 🙏 For the face 😶 , it is the lips 👄",
        something
    );

    ctx.say(glaze_txt).await?;
    Ok(())
}

pub async fn send_img_delete_msg(
    ctx: &serenity::Context,
    filename: &str,
    bytes: &[u8],
    msg: &Message,
) {
    let delete_fut = msg.delete(&ctx.http);
    let send_fut = msg.channel_id.send_files(
        &ctx.http,
        vec![CreateAttachment::bytes(bytes, filename)],
        CreateMessage::new().content(""),
    );

    let (_, send_result) = tokio::join!(delete_fut, send_fut);
    if let Err(e) = send_result {
        eprintln!("Failed to send file: {}", e);
    }
}

pub async fn send_gif_delete_msg(ctx: &serenity::Context, link: &str, msg: &Message) {
    let delete_fut = msg.delete(&ctx.http);
    let send_fut = msg.channel_id.say(&ctx.http, link);

    let (_, send_result) = tokio::join!(delete_fut, send_fut);
    if let Err(e) = send_result {
        eprintln!("Failed to send file: {}", e);
    }
}

pub async fn send_copypasta(ctx: &serenity::Context, key: &str, msg: &Message) {
    let &text = copypasta_lu()
        .get(key)
        .unwrap_or(&"Could not find corresponding copypasta");
    let chunks = chunk_str(text);
    for chunk in chunks {
        if let Err(e) = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::default().content(chunk))
            .await
        {
            eprintln!("Failed to send copypasta: {}", e);
        };
    }
}
