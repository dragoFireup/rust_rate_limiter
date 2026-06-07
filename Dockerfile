FROM rust:1.75-slim AS dev_build_stage
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=dev_build_stage /src/target/release/rust_rate_limiter .
CMD ["./rust_rate_limiter"]