use crate::internal::locate::{DEFAULT_SEED, MODDED_SEED};
use crate::utils::{driver::*, lus::*};

use poise::ChoiceParameter;
use reqwest;

#[derive(Debug, Clone, ChoiceParameter)]
enum ServerType {
    Modded,
    Unmodded,
}

pub fn unmodded(server_address: &str) -> String {
    format!(
        "**Unmodded Java Minecraft Server**\n\
        - Go to 'Multiplayer'\n\
        - Click 'Add Server'\n\
        - Address: {}\n\
        - World Spawn: (1371, 127, 765)\n\
        - Seed: {}\n\
        - Server Version: 1.21.7\n\
        *Please reach out to Trevor for any help!*\n\n\
        **_Please do not directly share the server's IP address :smile:_**",
        server_address,
        DEFAULT_SEED.to_string()
    )
}

pub fn modded(server_address: &str) -> String {
    format!(
        "**Modded Java Minecraft Server**\n\
        - Open Minecraft Launcher and Install Minecraft Version 1.18.2\n\
        - Download Forge for Minecraft 1.18.2 [Here](<https://files.minecraftforge.net/net/minecraftforge/forge/index_1.18.2.html>)\n\
          - Installer will bombard you with adds, just wait about 5 seconds and press 'Skip' in the top right corner!\n\
        - Run the executable jar, and select 'Install Client'\n\
        - Download the [CuresForge App](<https://www.curseforge.com/download/app>). \n\
        - Open the app and choose Minecraft as your game\n\
        - Add `Vault Hunters Official Modpack (Third Edition)` to the `My Modpacks` Tab\n\
        - Press Play. This will install a custom minecraft launcher that you must sign into.\n\
        - Once you get in the launcher, press play and agree to the privacy/security statement!\n\
        - Note that this is a custom instance of minecraft, and there may be some performance issues!\n\
        - Go to 'Multiplayer'\n\
        - Click 'Add Server'\n\
        - Address: {}\n\
        - World Spawn: (129, 63, -156)\n\
        - Seed: {}\n\
        - Server Version: 1.18.2\n\
        *Please reach out to Trevor for any help!*\n\n\
        **_Please do not directly share the server's IP address :smile:_**",
        server_address,
        MODDED_SEED.to_string()
    )
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Minecraft server information.")
)]
pub async fn server(
    ctx: Context<'_>,
    #[description = "The server to get help for"] server: ServerType,
) -> Result<(), Error> {
    let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
    let (address, modded_flag) = match server {
        ServerType::Modded => (format!("{}:9768", ip), true),
        ServerType::Unmodded => (format!("{}:9769", ip), false),
    };

    let msg = match modded_flag {
        true => modded(&address),
        false => unmodded(&address),
    };
    ctx.say(msg).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Lethal company modding information.")
)]
pub async fn lethal(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("**Lethal Company Modding Guide**\n\
        1. Download Thunderstore Mod Manager [here](<https://www.overwolf.com/app/Thunderstore-Thunderstore_Mod_Manager>)\n\
        2. Follow all setup steps and start the app\n\
        3. Select *Lethal Company* in the game menu and create a new profile\n\
        4. Install [this modpack](<https://thunderstore.io/c/lethal-company/p/KyoshiYoshi/RealAmogus/>) for peak lethal company gaming (Press *Install with Mod Manager*)\n\
        *Feel free to disable any mods you find annoying, but you must have the reserved item slot mods on if you want to take advantage of it.*").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Show the help menu.")
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let keys: Vec<&str> = misc_image_lu().keys().cloned().collect();
    let images = format!("Images:\n  {}", keys.join(", "));

    let keys: Vec<&str> = gif_lu().keys().cloned().collect();
    let gifs = format!("Gifs:\n  {}", keys.join(", "));

    let keys: Vec<&str> = copypasta_lu().keys().cloned().collect();
    let copypastas = format!("Copypastas:\n  {}", keys.join(", "));

    let extra_help = format!("{}\n\n{}\n\n{}", images, gifs, copypastas);

    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: &extra_help,
        ..Default::default()
    };
    poise::builtins::help(ctx, None, config).await?;
    Ok(())
}
