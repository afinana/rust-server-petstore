# === builder stage ==========================================================
FROM rust:1.82 AS builder
WORKDIR /app

# Copy manifest files and pull dependencies before copying sources so layers cache
COPY Cargo.lock Cargo.toml ./
RUN mkdir src && echo "fn main() { println!(\"loading deps...\"); }" > src/main.rs
RUN cargo build --release

# Copy actual source files and rebuild the release binary
COPY ./src ./src
RUN cargo build --release

# === runtime stage ==========================================================
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy final binary from builder image
COPY --from=builder /app/target/release/rust-server-petstore /rust-server-petstore

EXPOSE 8080
ENV RUST_LOG=actix_web=info

ENTRYPOINT ["/rust-server-petstore"]