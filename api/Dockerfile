FROM rust:1.87.0-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM rust:1.87.0-slim-bookworm AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/zerrenda-app-api .
CMD ["./zerrenda-app-api"]