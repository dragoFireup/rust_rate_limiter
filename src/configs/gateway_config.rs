use std::sync::atomic::Ordering;
use std::{
    collections::VecDeque,
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};

use arc_swap::ArcSwap;
use reqwest::Client;

use super::rate_limiter_gateway::RateLimiterGateway;
pub struct GatewayConfig {
    pub limiter: RateLimiterGateway,
    destinations: Vec<String>,
    active_destinations: ArcSwap<VecDeque<String>>,
    http_client: Client,
    current_destination_index: AtomicUsize,
}

impl GatewayConfig {
    pub async fn new(
        limiter: RateLimiterGateway,
        destinations: Vec<String>,
        http_client: Client,
    ) -> Self {
        let mut active_destinations = VecDeque::new();

        for destination in &destinations {
            if Self::check_health(&http_client, destination).await {
                println!("Adding the following host: {}", destination);
                active_destinations.push_back(destination.clone());
            }
        }

        Self {
            limiter,
            destinations,
            active_destinations: ArcSwap::new(Arc::new(active_destinations)),
            http_client,
            current_destination_index: AtomicUsize::new(0),
        }
    }

    async fn check_health(http_client: &Client, destination: &str) -> bool {
        let url = format!("{}/health", destination);

        http_client
            .get(url)
            .timeout(Duration::from_secs(2))
            .send()
            .await
            .and_then(|res| res.error_for_status())
            .is_ok()
    }

    pub fn get_destination_endpoint(&self) -> Option<String> {
        let active = self.active_destinations.load();

        if active.is_empty() {
            return None;
        }

        let current_idx = self
            .current_destination_index
            .fetch_add(1, Ordering::Relaxed);

        return Some(active[current_idx % active.len()].clone());
    }

    pub async fn run_health_check(&self) {
        let mut active = VecDeque::new();

        for destination in &self.destinations {
            if Self::check_health(&self.http_client, destination).await {
                active.push_back(destination.clone());
            }
        }

        self.active_destinations.store(Arc::new(active));
    }
}
