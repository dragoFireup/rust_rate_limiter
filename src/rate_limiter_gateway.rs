use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::time::Duration;

use crate::sliding_window_log::SlidingWindowLog;

pub struct RateLimiterGateway {
    window_size: Duration,
    max_requests: usize,
    client_registry: RwLock<HashMap<String, Mutex<SlidingWindowLog>>>,
}

impl RateLimiterGateway {
    pub fn new(window_size: Duration, max_requests: usize) -> Self {
        Self {
            window_size,
            max_requests,
            client_registry: RwLock::new(HashMap::new()),
        }
    }

    pub fn check_allowance(&self, client_id: &str) -> bool {
        {
            let registry = self.client_registry.read().unwrap();
            if let Some(client_log) = registry.get(client_id) {
                let mut log = client_log.lock().unwrap();
                return log.is_allowed(self.window_size, self.max_requests);
            }
        } // read lock is dropped here, else the lock would have been for the entire function

        let mut registry = self.client_registry.write().unwrap();

        let log_mutex = registry
            .entry(client_id.to_string())
            .or_insert_with(|| Mutex::new(SlidingWindowLog::new()));

        let mut log = log_mutex.lock().unwrap();
        log.is_allowed(self.window_size, self.max_requests)
    }
}
