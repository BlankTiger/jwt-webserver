mod models;
mod schema;

use color_eyre::Report;
use diesel::prelude::*;
use std::env;
use tracing::info;

fn setup() -> Result<(), Report> {
    use dotenvy::dotenv;
    use tracing_subscriber::EnvFilter;

    dotenv()?;

    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    color_eyre::install()?;

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn establish_connection() -> Result<SqliteConnection, Report> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)?)
}

fn main() -> Result<(), Report> {
    setup()?;
    let conn = establish_connection()?;
    info!("Everything is fine!");
    Ok(())
}
