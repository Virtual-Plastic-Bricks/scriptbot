use serde::{Serialize, Deserialize};
use serenity::model::id::*;
use std::path::Path;
use std::fs::File;
use std::io::Read;

// I'm designing this bot for single-server deployment
// God forbid should anything else happen.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub deployed_server_id: GuildId,
    pub question_channel: ChannelId
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let mut conf = String::new();
        File::open(path)
            .expect("Failed to open config file")
            .read_to_string(&mut conf)
            .expect("Failed to read config file");
        toml::from_str(&conf)
            .expect("Failed to parse config contents")
    }
}