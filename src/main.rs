mod bot;
mod config;
mod db;

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Client;

#[tokio::main]
async fn main() {
    let token = std::env::args().nth(1).expect("No token supplied");
    let mut client = Client::new(token)
        .event_handler(bot::Bot::new(
            config::Config::load("bot.toml"), // Move to ENV (both)
            db::sled::SledDB::new("bot.db")
        ))
        .intents(GatewayIntents::GUILD_MESSAGES)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}