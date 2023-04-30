use crate::db_actions;
use color_eyre::Result;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::EnvFilter;

pub async fn setup() -> Result<()> {
    dotenv()?;

    setup_error_handling()?;
    setup_tracing();
    setup_database().await?;

    Ok(())
}

fn setup_error_handling() -> Result<()> {
    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    color_eyre::install()?;
    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

async fn setup_database() -> Result<()> {
    let pool = db_actions::get_pool().await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(())
}
