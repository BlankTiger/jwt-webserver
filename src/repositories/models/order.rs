use chrono::NaiveDateTime;

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
