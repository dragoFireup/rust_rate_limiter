use axum::http::HeaderMap;

pub fn extract_client_identity(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("ClientId")
        .and_then(|value| value.to_str().ok())
}
