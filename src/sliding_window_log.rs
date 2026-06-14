use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct SlidingWindowLog {
    timestamps: VecDeque<Instant>,
}
