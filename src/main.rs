mod commands;
mod event_handlers;
mod utils;

use std::env;

use commands::{essentials::ESSENTIALS_GROUP, help::HELP, sounds::SOUNDS_GROUP};
use event_handlers::handler::MainEventHandler;
use serenity::{client::Client, framework::StandardFramework, prelude::GatewayIntents};

use songbird::SerenityInit;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&ESSENTIALS_GROUP)
        .group(&SOUNDS_GROUP)
        .help(&HELP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(MainEventHandler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
}
