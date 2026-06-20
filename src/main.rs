mod configs;
mod utils;

use axum::extract::{Request, State};
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::{body::Body, response::Response, routing::any, Router};
use reqwest::{Body as RBody, Client};
use std::sync::Arc;
use std::time::Duration;
use utils::client_identity_extractor::extract_client_identity;

use configs::gateway_config::GatewayConfig;
use configs::rate_limiter_gateway::RateLimiterGateway;

struct AppState {
    gateway_config: GatewayConfig,
    http_client: Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let http_client = Client::new();

    let gateway_config = GatewayConfig::new(
        RateLimiterGateway::new(Duration::from_secs(1), 10),
        vec![
            "http://localhost:5000".to_string(),
            "http://localhost:5001".to_string(),
        ],
        http_client.clone(),
    )
    .await;

    let shared_state = Arc::new(AppState {
        gateway_config: gateway_config,
        http_client: http_client,
    });

    let state_clone = shared_state.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            state_clone.gateway_config.run_health_check().await;
        }
    });

    let app = Router::new()
        .route("/", any(proxy_handler))
        .route("/{*path}", any(proxy_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    request: Request<Body>,
) -> Response {
    let Some(client_id) = extract_client_identity(request.headers()) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let is_allowed = state.gateway_config.limiter.check_allowance(client_id);

    if !is_allowed {
        return Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::from("Rate limit exceeded"))
            .unwrap();
    }

    let Some(destination) = state.gateway_config.get_destination_endpoint() else {
        return Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .body(Body::from("No healthy hosts available"))
            .unwrap();
    };

    let backend_url = format!("{}{}", destination, uri.path());

    let client = &state.http_client;

    let (parts, body) = request.into_parts();

    let proxy_request_body = RBody::wrap_stream(body.into_data_stream());

    let proxy_response = match client
        .request(parts.method, &backend_url)
        .headers(parts.headers)
        .body(proxy_request_body)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
    };

    let mut response_builder = Response::builder().status(proxy_response.status());

    *response_builder.headers_mut().unwrap() = proxy_response.headers().clone();

    return response_builder
        .body(Body::from_stream(proxy_response.bytes_stream()))
        .unwrap();
}
