use crate::utils::driver::*;

use std::process::{Command, Stdio};

use poise::{ChoiceParameter, serenity_prelude::CreateMessage};

const BINARY_PATH: &str = "libs/mclocate/zig-out/bin/mclocate";
const DEFAULT_SEED: i64 = 6304398765943131469;
const MAX_DISCORD_MSG_LENGTH: usize = 2000;

#[derive(Debug, Clone, ChoiceParameter)]
pub enum CommandOpt {
    #[name = "Biome"]
    Biome,
    #[name = "Structure"]
    Structure,
    #[name = "Biome Help"]
    HelpBiome,
    #[name = "Structure Help"]
    HelpStructure,
}

impl CommandOpt {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Biome => "biome",
            Self::Structure => "structure",
            Self::HelpBiome => "help-biome",
            Self::HelpStructure => "help-structure",
        }
    }
}

#[derive(Debug, Clone, ChoiceParameter)]
pub enum Dimension {
    #[name = "Overworld"]
    Overworld,
    #[name = "Nether"]
    Nether,
    #[name = "End"]
    End,
}

impl Dimension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Overworld => "o",
            Self::Nether => "n",
            Self::End => "e",
        }
    }
}

fn run_command(args: Vec<&str>) -> Result<String, Error> {
    let output = Command::new(BINARY_PATH)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let msg = if !stdout.is_empty() {
        stdout.trim()
    } else if !stderr.is_empty() {
        stderr.trim()
    } else {
        "No output"
    };
    Ok(msg.to_string())
}

fn chunk_str(s: &str) -> Vec<String> {
    s.as_bytes()
        .chunks(MAX_DISCORD_MSG_LENGTH)
        .map(|c| String::from_utf8_lossy(c).into_owned())
        .collect()
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Simulate the locate command on a minecraft seed")
)]
pub async fn locate(
    ctx: Context<'_>,
    #[description = "The type of operation you with to perform"] command: CommandOpt,
    #[description = "The seed to query, defaulting to the unmodded server's seed"] seed: Option<
        i64,
    >,
    #[description = "The dimension to search in, defaulting to the overworld"] dimension: Option<
        Dimension,
    >,
    #[description = "The string id of your query, use help-biome or help-structure for assistance"]
    id: Option<String>,
    #[description = "The x-coordinate to search at"] x: Option<i32>,
    #[description = "The z-coordinate to search at"] y: Option<i32>,
) -> Result<(), Error> {
    match command {
        CommandOpt::HelpBiome | CommandOpt::HelpStructure => {
            let msg = run_command(vec![command.as_str()])?;

            let chunks = chunk_str(&msg);
            for chunk in chunks {
                ctx.channel_id()
                    .send_message(
                        ctx.serenity_context(),
                        CreateMessage::default().content(chunk),
                    )
                    .await?;
            }

            let help_msg = "Usage: `/locate <biome|structure> <seed?> <dim> <id> <x> <z>` or `/locate help-biome|help-structure`";
            ctx.channel_id()
                .send_message(
                    ctx.serenity_context(),
                    CreateMessage::default().content(help_msg),
                )
                .await?;
            ctx.say("Responded in longform message").await?;
        }
        CommandOpt::Biome | CommandOpt::Structure => {
            let (seed, dimension, id, x, y) = (
                seed.unwrap_or(DEFAULT_SEED).to_string().as_str().to_owned(),
                dimension
                    .unwrap_or(Dimension::Overworld)
                    .as_str()
                    .to_owned(),
                id.ok_or("Must pass id for non-help queries")?
                    .as_str()
                    .to_owned(),
                x.ok_or("Must pass x-coordinate for non-help queries")?
                    .to_string()
                    .as_str()
                    .to_owned(),
                y.ok_or("Must pass y-coordinate for non-help queries")?
                    .to_string()
                    .as_str()
                    .to_owned(),
            );

            let args = vec![command.as_str(), &seed, &dimension, &id, &x, &y];
            let msg = run_command(args)?;
            ctx.say(msg).await?;
        }
    }

    Ok(())
}
