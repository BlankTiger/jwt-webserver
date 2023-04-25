use crate::db_actions;
use color_eyre::Report;
use std::env;

pub async fn setup() -> Result<(), Report> {
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

    let pool = db_actions::get_pool().await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(())
}
