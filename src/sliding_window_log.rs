use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct SlidingWindowLog {
    timestamps: VecDeque<Instant>,
}

impl SlidingWindowLog {
    pub fn new() -> Self {
        Self {
            timestamps: VecDeque::new()
        }
    }

    pub fn is_allowed(&mut self, window_size: Duration, max_requests: usize) -> bool {

        let now = Instant::now();

        let cutoff = now.checked_sub(window_size).unwrap();

        // evict all timestamps outside the cutoff window
        while let Some(&timestamp) = self.timestamps.front() {
            if timestamp < cutoff {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        if self.timestamps.len() < max_requests {
            self.timestamps.push_back(now);
            return true;
        }

        return false;

    } 
}