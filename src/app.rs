use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, patch, post, put},
    Router,
};
use color_eyre::Report;
use std::env;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};

use crate::{
    controllers::{customer_controller::*, order_controller::*, product_controller::*},
    db_actions::get_pool,
};

pub type DbPool = sqlx::PgPool;

pub async fn start_app() -> Result<(), Report> {
    let pool = get_pool().await?;
    let router = build_router().with_state(pool);
    let addr = env::var("SERVER_ADDR")?;

    axum::Server::bind(&addr.parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

fn build_router() -> Router<DbPool> {
    let api_routes = Router::new()
        .nest("/admin", admin_routes())
        .nest("/product", product_routes())
        .nest("/order", order_routes())
        .nest("/customer", customer_routes());

    add_error_handler(Router::new().nest("/api", api_routes))
}

fn add_error_handler(router: Router<DbPool>) -> Router<DbPool> {
    router.layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                if error.is::<tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                }
            }))
            .timeout(Duration::from_secs(10))
            .into_inner(),
    )
}

fn product_routes() -> Router<DbPool> {
    Router::new()
        .route("/:id", get(get_product))
        .route("/all", get(get_all_products))
}

fn customer_routes() -> Router<DbPool> {
    Router::new()
        .route("/:id", get(get_customer))
        .route("/all", get(get_all_customers))
}

fn order_routes() -> Router<DbPool> {
    Router::new()
        .route("/:id", get(get_order))
        .route("/all", get(get_all_orders))
}

fn admin_routes() -> Router<DbPool> {
    Router::new()
        .route("/product", post(create_product))
        .route("/product/:id", put(update_product))
        .route("/product/:id", patch(partial_update_product))
        .route("/customer", post(create_customer))
        .route("/customer/:id", put(update_customer))
        .route("/customer/:id", patch(partial_update_customer))
        .route("/order", post(create_order))
        .route("/order/:id", put(update_order))
        .route("/order/:id", patch(partial_update_order))
}
