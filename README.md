# Rust Rate Limiter Proxy

A high-performance, thread-safe rate limiting proxy implemented in Rust using the Axum web framework. This project serves as a robust implementation of the **Sliding Window Log** algorithm, designed to handle high-concurrency traffic with minimal overhead.

## 🚀 Overview

This repository implements a **Sliding Window Log** rate limiter that acts as a transparent proxy. It identifies clients via a `ClientId` header and enforces per-client rate limits before forwarding requests to a backend destination.

### Key Technical Highlights
- **Thread-Safe Architecture:** Uses a combination of `RwLock` for the client registry and granular `Mutex` locks for individual client logs, enabling high parallel throughput.
- **Sliding Window Log Algorithm:** Provides precise rate limiting by tracking individual request timestamps within a moving time window.
- **Zero-Allocation Fast Path:** Optimized lookup logic using `&str` and the Entry API to minimize heap allocations during hot paths.
- **Robust Time Handling:** Leverages `std::time::Instant` for monotonic time calculations, resilient against system clock shifts.

---

## 🛠 Features

- **Per-Client Rate Limiting:** Track and limit requests based on the `ClientId` HTTP header.
- **Transparent Proxying:** Forwards allowed requests to a configurable backend destination.
- **Granular Locking:** Uses "Double-Checked Locking" to ensure thread safety when adding new clients without blocking existing ones.
- **Dockerized:** Optimized Dockerfile for small image size and easy deployment.

---

## 📦 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Edition 2021)
- [Docker](https://docs.docker.com/get-docker/) (optional)

### Running Locally
```bash
# Clone the repository
git clone https://github.com/your-username/rust-rate-limiter.git
cd rust-rate-limiter

# Run the proxy (listens on port 3000)
cargo run
```

### Running with Docker
```bash
# Build the image
docker build -t rate-limiter:v2 .

# Run the container
docker run -d -p 3000:3000 --add-host=host.docker.internal:host-gateway --name operational-gateway rate_limiter:v2
```

Exceed the rate limit and you will notice 429 as a response
---

## 🗺 Roadmap

- [x] Basic Sliding Window Log implementation.
- [x] Dockerization of the Rust application.
- [x] **Middleware Support:** Integration with Axum proxy.
- [x] **ClientId based ratelimiter.**

---

## 📝 License

This project is open-source and available under the MIT License.
