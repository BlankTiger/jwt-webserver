use crate::db_actions::{get_pool, Clearable, MockFillable};

use super::models::product::*;
use super::PG_LIMIT;
use async_trait::async_trait;
use color_eyre::Report;
use sqlx::{PgPool, QueryBuilder};
use tracing::info;

pub struct ProductRepository;

#[async_trait]
impl MockFillable for ProductRepository {
    async fn fill_with_mocked_data(&self) -> Result<(), Report> {
        let new_products = [
            Product {
                name: "Product 1".to_string(),
                price: 1,
                available: true,
                ..Default::default()
            },
            Product {
                name: "Product 2".to_string(),
                price: 2,
                available: true,
                ..Default::default()
            },
        ];

        let pool = get_pool().await?;
        ProductRepository::create_products(&pool, &new_products).await?;
        Ok(())
    }
}

#[async_trait]
impl Clearable for ProductRepository {
    async fn clear(&self) -> Result<(), Report> {
        let pool = get_pool().await?;
        sqlx::query!("delete from products").execute(&pool).await?;
        Ok(())
    }
}

impl ProductRepository {
    pub async fn create_product(pool: &PgPool, new_product: Product) -> Result<(), Report> {
        sqlx::query("insert into products (name, price, available) values (?, ?, ?)")
            .bind(new_product.name)
            .bind(new_product.price)
            .bind(new_product.available)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn create_products(pool: &PgPool, new_products: &[Product]) -> Result<(), Report> {
        let mut query_builder = QueryBuilder::new("insert into products (name, price, available) ");
        query_builder.push_values(
            new_products.iter().take(PG_LIMIT as usize / 3),
            |mut builder, product| {
                builder
                    .push_bind(&product.name)
                    .push_bind(product.price)
                    .push_bind(product.available);
            },
        );

        info!("Executing group insert query: {}", query_builder.sql());
        let query = query_builder.build();
        query.execute(pool).await?;

        Ok(())
    }

    pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, Report> {
        Ok(sqlx::query_as!(Product, "select * from products")
            .fetch_all(pool)
            .await?)
    }
}
