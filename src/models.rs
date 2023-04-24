use chrono::NaiveDateTime;

#[derive(Debug, Hash, PartialEq, Eq, Default)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub available: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct ProductInOrder {
    pub product_id: i32,
    pub order_id: i32,
    pub quantity: i32,
}
