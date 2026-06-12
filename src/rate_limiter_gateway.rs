use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::time::Duration;

#[path = "sliding_window_log.rs"]
mod sliding_window_log;

use sliding_window_log::SlidingWindowLog;

pub struct RateLimiterGateway {
    window_size: Duration,
    max_request: usize,
    client_registry: RwLock<HashMap<String, Mutex<SlidingWindowLog>>>,
}

impl RateLimiterGateway {
    pub fn new(window_size: Duration, max_request: usize) -> Self {
        Self {
            window_size,
            max_request,
            client_registry: RwLock::new(HashMap::new()),
        }
    }
}
