use super::PG_LIMIT;
use crate::db_actions::{get_pool, Clearable, MockFillable};
use color_eyre::Report;
use sqlx::{PgPool, QueryBuilder};
use tracing::info;

use crate::models::customer::*;
use async_trait::async_trait;

pub struct CustomerService;

#[async_trait]
impl MockFillable for CustomerService {
    async fn fill_with_mocked_data(&self) -> Result<(), Report> {
        let new_customers = [
            Customer {
                name: "Customer 1".to_string(),
                address: "Address 1".to_string(),
                ..Default::default()
            },
            Customer {
                name: "Customer 2".to_string(),
                address: "Address 2".to_string(),
                ..Default::default()
            },
        ];

        let pool = get_pool().await?;
        CustomerService::create_customers(&pool, &new_customers).await?;
        Ok(())
    }
}

#[async_trait]
impl Clearable for CustomerService {
    async fn clear(&self) -> Result<(), Report> {
        let pool = get_pool().await?;
        sqlx::query!("delete from customers").execute(&pool).await?;
        Ok(())
    }
}

impl CustomerService {
    pub async fn create_customer(pool: &PgPool, new_customer: Customer) -> Result<(), Report> {
        sqlx::query!(
            "insert into customers (name, address) values ($1, $2)",
            new_customer.name,
            new_customer.address
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn create_customers(pool: &PgPool, new_customers: &[Customer]) -> Result<(), Report> {
        let mut query_builder = QueryBuilder::new("insert into customers (name, address) ");
        query_builder.push_values(
            new_customers.iter().take(PG_LIMIT as usize / 3),
            |mut builder, customer| {
                builder
                    .push_bind(&customer.name)
                    .push_bind(&customer.address);
            },
        );

        info!("Executing group insert query: {}", query_builder.sql());
        let query = query_builder.build();
        query.execute(pool).await?;

        Ok(())
    }

    pub async fn get_customer(pool: &PgPool, id: i32) -> Result<Customer, Report> {
        Ok(
            sqlx::query_as!(Customer, "select * from customers where id = $1", id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_all_customers(pool: &PgPool) -> Result<Vec<Customer>, Report> {
        Ok(sqlx::query_as!(Customer, "select * from customers")
            .fetch_all(pool)
            .await?)
    }

    pub async fn update_customer(pool: &PgPool, updated_customer: Customer) -> Result<(), Report> {
        sqlx::query!(
            "update customers set name = $1, address = $2 where id = $3",
            updated_customer.name,
            updated_customer.address,
            updated_customer.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
