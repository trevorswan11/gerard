use crate::utils::driver::*;

use std::io::Cursor;

use image::{ImageFormat, Luma};
use poise::serenity_prelude::CreateAttachment;
use qrcode::QrCode;

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Encode text into a qrcode")
)]
pub async fn qrcode(
    ctx: Context<'_>,
    #[rest]
    #[description = "The text to encode"]
    text: String,
) -> Result<(), Error> {
    let encoded = QrCode::new(text.as_bytes())?;
    let image = encoded
        .render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .max_dimensions(300, 300)
        .build();

    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, ImageFormat::Jpeg)?;
    let image_name = format!("{}-qrcode.jpg", ctx.author().id.get());

    ctx.send(
        poise::CreateReply::default()
            .attachment(CreateAttachment::bytes(buf.into_inner(), image_name)),
    )
    .await?;

    Ok(())
}
