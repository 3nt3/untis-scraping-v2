mod cli;
mod config;
mod untis;

use once_cell::sync::OnceCell;

use clap::Parser;
use config::Config;

static CONFIG: OnceCell<Config> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    CONFIG
        .set(
            config::read_config((&args.config).to_string())
                .expect(&format!("Error reading config file at {}", &args.config)),
        )
        .unwrap();

    let config = CONFIG.get().unwrap();

    let session_id = untis::api::login(
        (&config.untis.school).to_string(),
        (&config.untis.username).to_string(),
        (&config.untis.password).to_string(),
    )
    .await?;

    let token = untis::api::new_token(session_id).await?;
    dbg!(&token);

    return Ok(());
}
