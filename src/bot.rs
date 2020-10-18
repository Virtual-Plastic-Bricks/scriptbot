use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::config::Config;
use crate::db::DB;

pub struct Bot<Data: DB> {
    config: Config,
    data: Data
}

impl<Data: DB> Bot<Data> {
    pub fn new(config: Config, data: Data) -> Self {
        Bot { // Makes init simple if there's other internal fields
            config,
            data
        }
    }
}

#[async_trait]
impl<Data: DB> EventHandler for Bot<Data> {
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