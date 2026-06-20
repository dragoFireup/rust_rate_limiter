use super::rate_limiter_gateway::RateLimiterGateway;
pub struct GatewayConfig {
    pub limiter: RateLimiterGateway,
    pub destination: String,
}