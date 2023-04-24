use crate::models::*;
use async_trait::async_trait;
use chrono::Local;
use color_eyre::Report;
use sqlx::postgres::PgPool;
use sqlx::{QueryBuilder, Row};
use std::collections::hash_map::HashMap;
use std::env;
use tracing::info;

static PG_LIMIT: u16 = u16::MAX;

// TODO: move repositories to different files
// TODO: use cfg_if to use different pools for sqlite and postgres
pub async fn get_pool() -> Result<PgPool, Report> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(PgPool::connect(&database_url).await?)
}

// fn establish_connection() -> Result<PgConnection, Report> {
//     let database_url = env::var("DATABASE_URL")?;
//     Ok(PgConnection::establish(&database_url)?)
// }

#[async_trait]
trait MockFillable {
    async fn fill_with_mocked_data(&self) -> Result<(), Report>;
}

#[async_trait]
trait Clearable {
    async fn clear(&self) -> Result<(), Report>;
}

pub struct ProductRepository;
pub struct CustomerRepository;
pub struct OrderRepository;

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
            new_products.into_iter().take(PG_LIMIT as usize / 3),
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

impl CustomerRepository {
    pub async fn create_customer(pool: &PgPool, new_customer: Customer) -> Result<(), Report> {
        sqlx::query("insert into customers (name, address) values (?, ?)")
            .bind(new_customer.name)
            .bind(new_customer.address)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn create_customers(pool: &PgPool, new_customers: &[Customer]) -> Result<(), Report> {
        let mut query_builder = QueryBuilder::new("insert into customers (name, address) ");
        query_builder.push_values(
            new_customers.into_iter().take(PG_LIMIT as usize / 3),
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

    pub async fn get_all_customers(pool: &PgPool) -> Result<Vec<Customer>, Report> {
        Ok(sqlx::query_as!(Customer, "select * from customers")
            .fetch_all(pool)
            .await?)
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

    pub async fn get_all_orders(pool: &PgPool) -> Result<HashMap<Customer, Vec<Order>>, Report> {
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
impl MockFillable for CustomerRepository {
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
        CustomerRepository::create_customers(&pool, &new_customers).await?;
        Ok(())
    }
}

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
impl Clearable for ProductRepository {
    async fn clear(&self) -> Result<(), Report> {
        let pool = get_pool().await?;
        sqlx::query!("delete from products").execute(&pool).await?;
        Ok(())
    }
}

#[async_trait]
impl Clearable for CustomerRepository {
    async fn clear(&self) -> Result<(), Report> {
        let pool = get_pool().await?;
        sqlx::query!("delete from customers").execute(&pool).await?;
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

pub struct DbMockData {
    product_repository: ProductRepository,
    order_repository: OrderRepository,
    customer_repository: CustomerRepository,
}

impl DbMockData {
    pub fn new() -> Self {
        DbMockData {
            product_repository: ProductRepository,
            order_repository: OrderRepository,
            customer_repository: CustomerRepository,
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
