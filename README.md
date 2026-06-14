# Rust Rate Limiter

A high-performance, thread-safe rate limiter implemented in Rust. This project serves as a hands-on exploration of Rust concurrency primitives, Dockerization, and rate-limiting infrastructure.

## 🚀 Overview

This repository implements a **Sliding Window Log** rate limiting algorithm. It is designed to handle multi-threaded environments efficiently using Rust's `Arc` and `Mutex` primitives.

### Why this project?
- **Rust Learning:** Deep dive into ownership, threading, and synchronization.
- **Infrastructure:** Practice containerizing Rust applications using Docker.
- **Algorithms:** Implementation of common system design patterns for traffic shaping.

---

## 🛠 Features

- **Sliding Window Log Algorithm:** Provides precise rate limiting by tracking individual request timestamps.
- **Thread-Safe Implementation:** Built with `std::sync::Mutex` and `Arc` to support concurrent requests.
- **Docker Ready:** Includes a Dockerfile for easy deployment and testing.
- **Simulation Suite:** Built-in multi-threaded test simulation in `main.rs`.

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

# Run the simulation
cargo run
```

### Running with Docker
```bash
# Build the image
docker build -t rate-limiter:v1 .

# Run the container
docker run --rm rate-limiter:v1
```

---

## 🗺 Roadmap

This project is an evolving learning journey. Future milestones include:

- [x] Basic Sliding Window Log implementation.
- [x] Dockerization of the Rust application.
- [x] **Middleware Support:** Integration with web frameworks like Axum or Actix-web- Created a proxy handler.
- [ ] ClientId based ratelimiter


---

## 📝 License

This project is open-source and available under the MIT License.
