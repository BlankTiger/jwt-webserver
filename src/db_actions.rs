use crate::models::*;
use crate::schema::customers::dsl::*;
use crate::schema::orders::dsl::*;
use crate::schema::products::dsl::*;
use crate::schema::products_in_orders::dsl::*;
use chrono::Local;
use color_eyre::Report;
use diesel::prelude::*;
use std::collections::hash_map::HashMap;
use std::env;

fn establish_connection() -> Result<PgConnection, Report> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&database_url)?)
}

trait MockFillable {
    fn fill_with_mocked_data(&self) -> Result<(), Report>;
}

trait Clearable {
    fn clear(&self) -> Result<(), Report>;
}

pub struct ProductRepository;
pub struct CustomerRepository;
pub struct OrderRepository;

impl ProductRepository {
    pub fn create_products(new_products: &[NewProduct]) -> Result<(), Report> {
        let conn = &mut establish_connection()?;
        diesel::insert_into(products)
            .values(new_products)
            .execute(conn)?;

        Ok(())
    }

    pub fn get_all_products() -> Result<Vec<Product>, Report> {
        let conn = &mut establish_connection()?;
        Ok(products.select(Product::as_select()).load(conn)?)
    }
}

impl CustomerRepository {
    pub fn create_customers(new_customers: &[NewCustomer]) -> Result<(), Report> {
        let conn = &mut establish_connection()?;
        diesel::insert_into(customers)
            .values(new_customers)
            .execute(conn)?;

        Ok(())
    }

    pub fn get_all_customers() -> Result<Vec<Customer>, Report> {
        let conn = &mut establish_connection()?;
        Ok(customers.select(Customer::as_select()).load(conn)?)
    }
}

impl OrderRepository {
    pub fn create_orders(
        customer_orders: &HashMap<NewOrder, HashMap<&Product, i32>>,
    ) -> Result<(), Report> {
        let conn = &mut establish_connection()?;
        for (new_order, products_in_order) in customer_orders.iter() {
            let curr_order_id = diesel::insert_into(orders)
                .values(new_order)
                .returning(crate::schema::orders::id)
                .get_result(conn)?;

            let products_in_order: Vec<NewProductInOrder> = products_in_order
                .iter()
                .map(|(product, amount)| NewProductInOrder {
                    order_id: &curr_order_id,
                    product_id: &product.id,
                    quantity: amount,
                })
                .collect();

            diesel::insert_into(products_in_orders)
                .values(&products_in_order)
                .execute(conn)?;
        }

        Ok(())
    }

    pub fn get_all_orders() -> Result<Vec<(Customer, Vec<Order>)>, Report> {
        let conn = &mut establish_connection()?;
        let all_customers = crate::schema::customers::table.select(Customer::as_select()).load(conn)?;
        let customer_orders = Order::belonging_to(&all_customers)
            .select(Order::as_select())
            .load(conn)?
            .grouped_by(&all_customers);

        Ok(all_customers.into_iter().zip(customer_orders).collect())
    }
}

impl MockFillable for ProductRepository {
    fn fill_with_mocked_data(&self) -> Result<(), Report> {
        let new_products = [
            NewProduct {
                name: "Product 1",
                price: &1,
                available: &true,
            },
            NewProduct {
                name: "Product 2",
                price: &2,
                available: &true,
            },
        ];
        ProductRepository::create_products(&new_products)?;
        Ok(())
    }
}

impl MockFillable for CustomerRepository {
    fn fill_with_mocked_data(&self) -> Result<(), Report> {
        let new_customers = [
            NewCustomer {
                name: "Customer 1",
                address: "Address 1",
            },
            NewCustomer {
                name: "Customer 2",
                address: "Address 2",
            },
        ];
        CustomerRepository::create_customers(&new_customers)?;

        Ok(())
    }
}

impl MockFillable for OrderRepository {
    fn fill_with_mocked_data(&self) -> Result<(), Report> {
        let mut customer_orders = HashMap::new();
        let products_in_db = ProductRepository::get_all_products()?;
        let customers_in_db = CustomerRepository::get_all_customers()?;

        let new_order = NewOrder {
            customer_id: &customers_in_db[0].id,
            status: "In progress",
            created_at: &Local::now().naive_local(),
        };
        let mut products_in_order = HashMap::new();
        products_in_order.insert(&products_in_db[0], 1);
        products_in_order.insert(&products_in_db[1], 2);
        customer_orders.insert(new_order, products_in_order);

        let new_order = NewOrder {
            customer_id: &customers_in_db[0].id,
            status: "In progress",
            created_at: &Local::now().naive_local(),
        };
        let mut products_in_order = HashMap::new();
        products_in_order.insert(&products_in_db[0], 6);
        products_in_order.insert(&products_in_db[1], 2);
        customer_orders.insert(new_order, products_in_order);

        let new_order = NewOrder {
            customer_id: &customers_in_db[1].id,
            status: "New",
            created_at: &Local::now().naive_local(),
        };
        let mut products_in_order = HashMap::new();
        products_in_order.insert(&products_in_db[0], 3);
        products_in_order.insert(&products_in_db[1], 4);
        customer_orders.insert(new_order, products_in_order);
        OrderRepository::create_orders(&customer_orders)?;

        Ok(())
    }
}

impl Clearable for ProductRepository {
    fn clear(&self) -> Result<(), Report> {
        let conn = &mut establish_connection()?;
        diesel::delete(products).execute(conn)?;
        Ok(())
    }
}

impl Clearable for CustomerRepository {
    fn clear(&self) -> Result<(), Report> {
        let conn = &mut establish_connection()?;
        diesel::delete(customers).execute(conn)?;
        Ok(())
    }
}

impl Clearable for OrderRepository {
    fn clear(&self) -> Result<(), Report> {
        let conn = &mut establish_connection()?;
        diesel::delete(products_in_orders).execute(conn)?;
        diesel::delete(orders).execute(conn)?;
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

    pub fn fill(&self) -> Result<(), Report> {
        self.customer_repository.fill_with_mocked_data()?;
        self.product_repository.fill_with_mocked_data()?;
        self.order_repository.fill_with_mocked_data()?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), Report> {
        self.order_repository.clear()?;
        self.customer_repository.clear()?;
        self.product_repository.clear()?;
        Ok(())
    }
}
