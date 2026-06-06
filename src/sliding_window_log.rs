use::std::collections::VecDeque;
use::std::time::{Duration, Instant};

pub struct SlidingWindowLog {
    window_size: Duration,
    max_requests: usize,
    log: VecDeque<Instant>, // Our chronological timeline history
}