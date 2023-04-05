use crate::models::{NewProduct, Product};
use crate::schema::products::dsl::*;
use color_eyre::Report;
use diesel::prelude::*;
use std::env;

fn establish_connection() -> Result<SqliteConnection, Report> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)?)
}

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
