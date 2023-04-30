use crate::db_actions::{get_pool, Clearable, MockFillable};

use super::PG_LIMIT;
use crate::models::Product;
use async_trait::async_trait;
use color_eyre::Result;
use sqlx::{PgPool, QueryBuilder};
use tracing::info;

pub struct ProductService;

#[async_trait]
impl MockFillable for ProductService {
    async fn fill_with_mocked_data(&self) -> Result<()> {
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
        ProductService::create_products(&pool, &new_products).await?;
        Ok(())
    }
}

#[async_trait]
impl Clearable for ProductService {
    async fn clear(&self) -> Result<()> {
        let pool = get_pool().await?;
        sqlx::query!("delete from products").execute(&pool).await?;
        Ok(())
    }
}

impl ProductService {
    pub async fn create_product(pool: &PgPool, new_product: Product) -> Result<i32> {
        let new_product_row: (i32,) = sqlx::query_as(
            "insert into products (name, price, available) values ($1, $2, $3) returning id",
        )
        .bind(new_product.name)
        .bind(new_product.price)
        .bind(new_product.available)
        .fetch_one(pool)
        .await?;

        Ok(new_product_row.0)
    }

    pub async fn update_product(pool: &PgPool, updated_product: Product) -> Result<()> {
        sqlx::query!(
            "update products set name = $1, price = $2, available = $3 where id = $4",
            updated_product.name,
            updated_product.price,
            updated_product.available,
            updated_product.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn create_products(pool: &PgPool, new_products: &[Product]) -> Result<()> {
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

    pub async fn get_product(pool: &PgPool, id: i32) -> Result<Product> {
        Ok(
            sqlx::query_as!(Product, "select * from products where id = $1", id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>> {
        Ok(sqlx::query_as!(Product, "select * from products")
            .fetch_all(pool)
            .await?)
    }
}
