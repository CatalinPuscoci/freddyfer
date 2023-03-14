use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub command_prefix: String,
}
