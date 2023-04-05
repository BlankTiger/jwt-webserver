use color_eyre::Report;
use tracing::info;

fn setup() -> Result<(), Report> {
    use dotenvy::dotenv;
    use std::env::{set_var, var};
    use tracing_subscriber::EnvFilter;

    dotenv()?;

    if var("RUST_LIB_BACKTRACE").is_err() {
        set_var("RUST_LIB_BACKTRACE", "1");
    }

    if var("RUST_LOG").is_err() {
        set_var("RUST_LOG", "info");
    }

    color_eyre::install()?;

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn main() -> Result<(), Report> {
    setup()?;
    info!("Everything is fine!");
    Ok(())
}
