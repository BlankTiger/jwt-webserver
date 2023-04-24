mod db_actions;
mod models;

use color_eyre::Report;
use db_actions::{DbMockData, OrderRepository};
use std::env;
use tracing::info;

// TODO: move this to setup.rs
async fn setup() -> Result<(), Report> {
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

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup().await?;

    let db_mock_data = DbMockData::new();
    db_mock_data.clear().await?;
    db_mock_data.fill().await?;

    let pool = &db_actions::get_pool().await?;
    let all_orders = OrderRepository::get_all_orders(pool).await?;
    for (customer, customers_orders) in all_orders.iter() {
        info!("Orders for customer <{:?}>:", customer);
        for order in customers_orders {
            info!("    - {:?}", order);
        }
    }

    Ok(())
}
