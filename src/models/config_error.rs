use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
#[snafu(module)]
pub enum ConfigError {
    #[snafu(display("config.json not found. Reason: {}", source))]
    NotFound { source: std::io::Error },
    #[snafu(display("Could not read from file. Reason: {}", source))]
    ReadFail { source: std::io::Error },
    #[snafu(display("Could not convert from JSON to Config. Reason: {}", source))]
    JsonConvertFail { source: serde_json::Error },
}
