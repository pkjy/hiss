use axum::routing::{get, post};

use crate::handler;

pub fn init() -> axum::Router {
    axum::Router::new()
        .route(
            "/company",
            post(handler::company::add).get(handler::company::list),
        )
        .route(
            "/short_url",
            post(handler::short_url::add).get(handler::short_url::list),
        )
        .route("/short_domain", post(handler::short_domain::add))
        .route("/:id", get(handler::short_url::do_redirect))
}
