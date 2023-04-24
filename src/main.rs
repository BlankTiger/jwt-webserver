use color_eyre::Report;
use data::db_actions::{get_pool, DbMockData};
use data::setup::setup;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup().await?;

    let db_mock_data = DbMockData::new();
    db_mock_data.clear().await?;
    db_mock_data.fill().await?;

    let pool = &get_pool().await?;
    let all_orders = db_mock_data.order_repository.get_all_orders(pool).await?;
    for (customer, customers_orders) in all_orders.iter() {
        info!("Orders for customer <{:?}>:", customer);
        for order in customers_orders {
            info!("    - {:?}", order);
        }
    }

    Ok(())
}
