// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        name -> Text,
        address -> Text,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        customer_id -> Int4,
        status -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Text,
        price -> Float4,
        available -> Bool,
    }
}

diesel::table! {
    products_in_orders (order_id, product_id) {
        order_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
    }
}

diesel::joinable!(orders -> customers (customer_id));
diesel::joinable!(products_in_orders -> orders (order_id));
diesel::joinable!(products_in_orders -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    orders,
    products,
    products_in_orders,
);
