use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub available: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub price: &'a f32,
    pub available: &'a bool,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::customers)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub address: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::customers)]
pub struct NewCustomer<'a> {
    pub name: &'a str,
    pub address: &'a str,
}

#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(belongs_to(Customer))]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder<'a> {
    pub customer_id: &'a i32,
    pub status: &'a str,
    pub created_at: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(table_name = crate::schema::products_in_orders)]
#[diesel(belongs_to(Product))]
#[diesel(belongs_to(Order))]
#[diesel(primary_key(product_id, order_id))]
pub struct ProductInOrder {
    pub product_id: i32,
    pub order_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::products_in_orders)]
pub struct NewProductInOrder<'a> {
    pub product_id: &'a i32,
    pub order_id: &'a i32,
    pub quantity: &'a i32,
}