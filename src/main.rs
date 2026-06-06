use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod sliding_window_log;

use sliding_window_log::SlidingWindowLog;

fn main() {
    let limiter = Arc::new(SlidingWindowLog::new(Duration::from_secs(1), 3)); // 3 TPS

    let mut execution_handle = vec![];

    println!("[INIT] Simulating 6 multi-threaded calls to the service..");

    for task_id in 1..6 {
        let limiter_ref = Arc::clone(&limiter);

        let handle = thread::spawn(move || {
            if limiter_ref.acquire() {
                println!("[THREAD#{:02}] Request processed state: SUCCESSFUL", task_id);
            } else {
                println!("[THREAD#{:02}] Request processed state: FAILED", task_id);
            }
        });

        execution_handle.push(handle);
    }

    for handle in execution_handle {
        handle.join().unwrap();
    }

    println!("Rate limiting multi-threaded simulation completed.");
}
