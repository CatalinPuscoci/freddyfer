mod commands;
mod event_handlers;
mod models;
mod utils;

use std::{env, fs::File, io::Read};

use commands::{
    essentials::ESSENTIALS_GROUP, funny::FUNNY_GROUP, help::HELP, sounds::SOUNDS_GROUP,
};
use event_handlers::handler::MainEventHandler;
use models::{config::Config, config_error::*};
use serenity::{client::Client, framework::StandardFramework, prelude::GatewayIntents};

use snafu::ResultExt;
use songbird::SerenityInit;

fn read_config() -> Result<Config, ConfigError> {
    let mut config_json_file = File::open("config.json").context(config_error::NotFoundSnafu)?;

    let mut json: String = Default::default();
    config_json_file
        .read_to_string(&mut json)
        .context(config_error::ReadFailSnafu)?;

    let cfg: Config =
        serde_json::from_str(json.as_str()).context(config_error::JsonConvertFailSnafu)?;

    Ok(cfg)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token in the environment.
    let config = match read_config() {
        Ok(cfg) => cfg,
        Err(why) => {
            eprintln!(
                "Could not read config, falling back on default. Reason: {}",
                why
            );

            Config {
                bot_token: env::var("DISCORD_TOKEN").expect("Expected a token in the environment"),
                command_prefix: ".".to_string(),
            }
        }
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.command_prefix))
        .group(&ESSENTIALS_GROUP)
        .group(&FUNNY_GROUP)
        .group(&SOUNDS_GROUP)
        .help(&HELP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&config.bot_token, intents)
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
