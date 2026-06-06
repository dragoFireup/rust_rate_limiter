use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::sync::Mutex;

pub struct SlidingWindowLog {
    window_size: Duration,
    max_requests: usize,
    log: Mutex<VecDeque<Instant>>, // Our chronological timeline history
}

impl SlidingWindowLog {
    pub fn new(window_size: Duration, max_requests: usize) -> Self {
        Self {
            window_size, 
            max_requests, 
            log: Mutex::new(VecDeque::new()), // thread safe
        }
    }

    pub fn acquire(&self) -> bool {

        let mut log = self.log.lock().unwrap();

        let now = Instant::now();

        let cutoff = now.checked_sub(self.window_size).unwrap();

        // evict all timestamps outside the cutoff window
        while let Some(&timestamp) = log.front() {
            if timestamp < cutoff {
                log.pop_front();
            } else {
                break;
            }
        }

        if log.len() < self.max_requests {
            log.push_back(now);
            return true;
        }
        return false;
    }
}