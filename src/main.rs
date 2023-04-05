mod db_actions;
mod models;
mod schema;

use color_eyre::Report;
use db_actions::{create_products, get_all_products};
use models::NewProduct;
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

fn main() -> Result<(), Report> {
    setup()?;
    let new_products = vec![
        NewProduct {
            name: "Product 1",
            price: &1.0,
            available: &true,
        },
        NewProduct {
            name: "Product 2",
            price: &2.0,
            available: &false,
        },
        NewProduct {
            name: "Product 3",
            price: &3.0,
            available: &true,
        },
    ];

    create_products(&new_products)?;
    let products = get_all_products()?;

    for product in products {
        info!("{:?}", product);
    }

    Ok(())
}
