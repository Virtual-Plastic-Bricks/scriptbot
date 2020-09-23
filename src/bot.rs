use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::config::Config;

pub struct Bot {
    config: Config
}

impl Bot {
    pub fn new(config: Config) -> Self {
        Bot { // Makes init simple if there's other internal fields
            config
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _ctx: Context, message: Message) {
        // Is the message in the question-asking channel?
        // Yes? Perfect. Check that it's a question. If it's not a question, it's a mini-mute.
        // Create new channel for Q
        // Request more information (if needed) and open to public once available
        // Look for: answer cmd, close cmd, accept cmd
        // When answer accepted, delete channel (w/ saved results) + open Q chan to user again

        unimplemented!()
    }
}