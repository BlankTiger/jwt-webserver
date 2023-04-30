use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, patch, post, put},
    Router,
};
use color_eyre::Result;
use std::env;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};

use crate::{controllers::*, db_actions::get_pool};

pub type DbPool = sqlx::PgPool;
pub struct App;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub async fn start_app(self) -> Result<()> {
        let pool = get_pool().await?;
        let router = self.build_router().with_state(pool);
        let addr = env::var("SERVER_ADDR")?;

        axum::Server::bind(&addr.parse()?)
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }

    fn build_router(self) -> Router<DbPool> {
        let api_routes = Router::new()
            .nest("/admin", Routes::admin_routes())
            .nest("/product", Routes::product_routes())
            .nest("/order", Routes::order_routes())
            .nest("/customer", Routes::customer_routes());

        self.add_error_handler(Router::new().nest("/api", api_routes))
    }

    fn add_error_handler(self, router: Router<DbPool>) -> Router<DbPool> {
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

}

struct Routes;

impl Routes {
    fn product_routes() -> Router<DbPool> {
        Router::new()
            .route("/:id", get(ProductController::get_product))
            .route("/all", get(ProductController::get_all_products))
    }

    fn customer_routes() -> Router<DbPool> {
        Router::new()
            .route("/:id", get(CustomerController::get_customer))
            .route("/all", get(CustomerController::get_all_customers))
    }

    fn order_routes() -> Router<DbPool> {
        Router::new()
            .route("/:id", get(OrderController::get_order))
            .route("/all", get(OrderController::get_all_orders))
    }

    fn admin_routes() -> Router<DbPool> {
        Router::new()
            .route("/product", post(ProductController::create_product))
            .route("/product/:id", put(ProductController::update_product))
            .route("/product/:id", patch(ProductController::partial_update_product))
            .route("/customer", post(CustomerController::create_customer))
            .route("/customer/:id", put(CustomerController::update_customer))
            .route("/customer/:id", patch(CustomerController::partial_update_customer))
            .route("/order", post(OrderController::create_order))
            .route("/order/:id", put(OrderController::update_order))
            .route("/order/:id", patch(OrderController::partial_update_order))
    }
}
