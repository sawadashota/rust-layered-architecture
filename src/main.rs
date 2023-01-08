#![allow(dead_code)]

mod domain;
mod infrastructure;
mod routes;
mod usecase;
mod registry;
mod middleware;
mod logger;

use std::sync::Arc;
use std::time::Duration;
use axum::{routing::{get}, Router};
use axum::http::{Request};
use axum::response::Response;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::request_id::MakeRequestUuid;
use tower_http::ServiceBuilderExt;
use tracing::Span;
use crate::registry::Registry;

#[tokio::main]
async fn main() {
    logger::setup(log::LevelFilter::Info);

    let registry = Arc::new(Registry::new());

    let user_router = Router::new()
        .route("/",
               get(routes::user::list_users)
                   .post(routes::user::create_user))
        .route("/:id",
               get(routes::user::find_user)
                   .delete(routes::user::delete_user)
                   .put(routes::user::update_user));

    let health_router = Router::new()
        .route("/alive", get(routes::health::alive));

    let app = Router::new()
        .nest("/users", user_router)
        .nest("/health", health_router)
        .with_state(registry)
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .on_request(|request: &Request<_>, _span: &Span| {
                            // let _a = request.headers().get("x-request-id").map(|v| v.to_str().unwrap().to_string());
                            tracing::info!(
                            "{}",
                            serde_json::json!(logger::Payload {
                                x_request_id: request.headers().get("x-request-id").map(|v| v.to_str().unwrap().to_string()),
                                host: request.headers().get("host").map(|v| v.to_str().unwrap().to_string()),
                                user_agent: request.headers().get("user-agent").map(|v| v.to_str().unwrap().to_string()),
                                method: Some(request.method().to_string()),
                                uri: Some(request.uri().to_string()),
                                status: None,
                                duration: None,
                                kind: logger::LogKind::Request,
                                error_message: None
                            })
                            .to_string()
                        )
                        })
                        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                            tracing::info!(
                            "{}",
                            serde_json::json!(logger::Payload {
                                x_request_id: response.headers().get("x-request-id").map(|v| v.to_str().unwrap().to_string()),
                                status: Some(response.status().as_str().to_string()),
                                duration: Some(latency.as_nanos().try_into().unwrap()),
                                host: None,
                                user_agent: None,
                                method: None,
                                uri: None,
                                kind: logger::LogKind::Response,
                                error_message: None
                            })
                            .to_string()
                        )
                        })
                        .on_body_chunk(())
                        .on_eos(())
                )
                .propagate_x_request_id(),
        );

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
