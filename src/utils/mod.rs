pub mod driver;
pub mod lus;

const MAX_DISCORD_MSG_LENGTH: usize = 2000;

pub fn chunk_str(s: &str) -> Vec<String> {
    s.as_bytes()
        .chunks(MAX_DISCORD_MSG_LENGTH)
        .map(|c| String::from_utf8_lossy(c).into_owned())
        .collect()
}
