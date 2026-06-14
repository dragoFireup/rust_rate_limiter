FROM rust:1.96.0-slim-bookworm AS dev_build_stage
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=dev_build_stage /src/target/release/rust_rate_limiter .
CMD ["./rust_rate_limiter"]