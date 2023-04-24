use async_trait::async_trait;
use color_eyre::Report;
use sqlx::postgres::PgPool;
use std::env;

use crate::repositories::{
    customer_repo::CustomerRepository, order_repo::OrderRepository, product_repo::ProductRepository,
};

// TODO: use cfg_if to use different pools for sqlite and postgres
pub async fn get_pool() -> Result<PgPool, Report> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(PgPool::connect(&database_url).await?)
}

#[async_trait]
pub trait MockFillable {
    async fn fill_with_mocked_data(&self) -> Result<(), Report>;
}

#[async_trait]
pub trait Clearable {
    async fn clear(&self) -> Result<(), Report>;
}

pub struct DbMockData {
    pub product_repository: ProductRepository,
    pub order_repository: OrderRepository,
    pub customer_repository: CustomerRepository,
}

impl Default for DbMockData {
    fn default() -> Self {
        Self::new()
    }
}

impl DbMockData {
    pub fn new() -> Self {
        DbMockData {
            product_repository: ProductRepository {},
            order_repository: OrderRepository {},
            customer_repository: CustomerRepository {},
        }
    }

    pub async fn fill(&self) -> Result<(), Report> {
        self.customer_repository.fill_with_mocked_data().await?;
        self.product_repository.fill_with_mocked_data().await?;
        self.order_repository.fill_with_mocked_data().await?;
        Ok(())
    }

    pub async fn clear(&self) -> Result<(), Report> {
        self.order_repository.clear().await?;
        self.customer_repository.clear().await?;
        self.product_repository.clear().await?;
        Ok(())
    }
}
