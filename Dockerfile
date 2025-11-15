# Rust as the base image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/rust_server_petstore*
RUN cargo build --release

# The final base image
FROM debian:bookworm-slim
#FROM scratch

# Copy from the previous build
COPY --from=build /app/target/release/rust-server-petstore /rust-server-petstore
# COPY --from=build /holodeck/target/release/holodeck/target/x86_64-unknown-linux-musl/release/holodeck .

# Run the binary
CMD ["/rust-server-petstore"]