use crate::db_actions::{get_pool, Clearable, MockFillable};
use chrono::Local;
use color_eyre::Report;
use sqlx::{PgPool, QueryBuilder};
use std::collections::HashMap;
use tracing::info;

use super::PG_LIMIT;

use super::customer_repo::CustomerRepository;
use super::models::{customer::*, order::*, product::*};
use super::product_repo::ProductRepository;
use async_trait::async_trait;

pub struct OrderRepository;

#[async_trait]
impl MockFillable for OrderRepository {
    async fn fill_with_mocked_data(&self) -> Result<(), Report> {
        let pool = get_pool().await?;
        let mut customer_orders = HashMap::new();
        let products_in_db = ProductRepository::get_all_products(&pool).await?;
        let customers_in_db = CustomerRepository::get_all_customers(&pool).await?;

        let new_order = Order {
            customer_id: customers_in_db[0].id,
            status: "In progress".to_string(),
            created_at: Local::now().naive_local(),
            ..Default::default()
        };
        let mut products_in_order = HashMap::new();
        products_in_order.insert(&products_in_db[0], 1);
        products_in_order.insert(&products_in_db[1], 2);
        customer_orders.insert(new_order, products_in_order);

        let new_order = Order {
            customer_id: customers_in_db[0].id,
            status: "In progress".to_string(),
            created_at: Local::now().naive_local(),
            ..Default::default()
        };
        let mut products_in_order = HashMap::new();
        products_in_order.insert(&products_in_db[0], 6);
        products_in_order.insert(&products_in_db[1], 2);
        customer_orders.insert(new_order, products_in_order);

        let new_order = Order {
            customer_id: customers_in_db[1].id,
            status: "New".to_string(),
            created_at: Local::now().naive_local(),
            ..Default::default()
        };
        let mut products_in_order = HashMap::new();
        products_in_order.insert(&products_in_db[0], 3);
        products_in_order.insert(&products_in_db[1], 4);
        customer_orders.insert(new_order, products_in_order);
        OrderRepository::create_orders(&pool, &customer_orders).await?;

        Ok(())
    }
}

#[async_trait]
impl Clearable for OrderRepository {
    async fn clear(&self) -> Result<(), Report> {
        let pool = get_pool().await?;
        sqlx::query!("delete from products_in_orders")
            .execute(&pool)
            .await?;
        sqlx::query!("delete from orders").execute(&pool).await?;
        Ok(())
    }
}

impl OrderRepository {
    pub async fn create_orders(
        pool: &PgPool,
        customer_orders: &HashMap<Order, HashMap<&Product, i32>>,
    ) -> Result<(), Report> {
        for (new_order, products_in_order) in customer_orders.iter() {
            let curr_order_row: (i32,) = sqlx::query_as(
                "insert into orders (customer_id, status, created_at) values ($1, $2, $3) returning id",
            )
                .bind(new_order.customer_id)
                .bind(&new_order.status)
                .bind(new_order.created_at)
                .fetch_one(pool)
                .await?;
            let curr_order_id = curr_order_row.0;

            let products_in_order: Vec<ProductInOrder> = products_in_order
                .iter()
                .map(|(product, amount)| ProductInOrder {
                    order_id: curr_order_id,
                    product_id: product.id,
                    quantity: *amount,
                })
                .collect();

            let mut query_builder = QueryBuilder::new(
                "insert into products_in_orders (order_id, product_id, quantity) ",
            );
            query_builder.push_values(
                products_in_order.into_iter().take(PG_LIMIT as usize / 3),
                |mut builder, product_in_order| {
                    builder
                        .push_bind(product_in_order.order_id)
                        .push_bind(product_in_order.product_id)
                        .push_bind(product_in_order.quantity);
                },
            );

            info!("Executing group insert query: {}", query_builder.sql());
            let query = query_builder.build();
            query.execute(pool).await?;
        }

        Ok(())
    }

    pub async fn get_all_orders(self, pool: &PgPool) -> Result<HashMap<Customer, Vec<Order>>, Report> {
        let mut all_orders = HashMap::new();
        let all_customers = sqlx::query_as!(Customer, "select * from customers")
            .fetch_all(pool)
            .await?;

        for customer in all_customers {
            let customer_orders = sqlx::query_as!(
                Order,
                "select * from orders where customer_id = $1",
                customer.id
            )
            .fetch_all(pool)
            .await?;
            all_orders.insert(customer, customer_orders);
        }

        Ok(all_orders)
    }
}
