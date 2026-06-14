mod rate_limiter_gateway;
pub mod sliding_window_log;

use ::axum::{body::Body, response::Response, routing::any, Router};
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::IntoResponse;
use reqwest::{Body as RBody, Client};
use std::sync::Arc;
use std::time::Duration;

use rate_limiter_gateway::RateLimiterGateway;

struct GatewayConfig {
    limiter: RateLimiterGateway,
    destination: String,
}

struct AppState {
    gateway_config: GatewayConfig,
    http_client: Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let gateway_config = GatewayConfig {
        limiter: RateLimiterGateway::new(Duration::from_secs(1), 10),
        destination: "http://host.docker.internal.:5000".to_string(),
    };

    let http_client = Client::new();

    let shared_state = Arc::new(AppState {
        gateway_config: gateway_config,
        http_client: http_client,
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

    let backend_url = format!("{}{}", state.gateway_config.destination, uri.path());

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

fn extract_client_identity(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("ClientId")
        .and_then(|value| value.to_str().ok())
        .map(|s| s)
}
