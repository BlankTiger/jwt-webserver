mod db_actions;
mod models;
mod schema;

use color_eyre::Report;
use db_actions::{DbMockData, OrderRepository};
use tracing::info;
use std::env;

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

fn main() -> Result<(), Report> {
    setup()?;

    let db_mock_data = DbMockData::new();
    db_mock_data.clear()?;
    db_mock_data.fill()?;

    let all_orders = OrderRepository::get_all_orders()?;
    for customer_order in all_orders {
        info!("Orders for customer <{:?}>:", customer_order.0);
        for order in customer_order.1 {
            info!("    - {:?}", order);
        }
    }

    Ok(())
}
