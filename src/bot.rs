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
        // Is the message from a user?
        if message.author.bot {return}
        // Is it in the guild?
        if message.guild_id != Some(self.config.deployed_server_id) {return}
        // Is the message in the question-asking channel?
        if message.channel_id != self.config.question_channel {return}
        // Yes? Perfect. Check that it's a question. If it's not a question, it's a mini-mute.
        // Simple check for question mark for now. We can get smarter later.
        if message.content.contains("?") {
            // Create new channel for Q
            self.config.deployed_server_id.create_channel(_ctx, |chan|chan
                .name("question") // Please work out how to generate these better
                // .topic(asked_question)
                // .category(self.config.question_category)
                .kind(ChannelType::Text)
                // .permissions(new_user_perms) // Hide all + show owner + show bot
            ).await;
            // Request more information (if needed) and open to public once available
            // Look for: answer cmd, close cmd, accept cmd
            // When answer accepted, delete channel (w/ saved results) + open Q chan to user again
        } else {
            // Mini-mute for shitty non-question
            // Tell the user why. Feed them their question for good measure.
        }
        unimplemented!()
    }
}