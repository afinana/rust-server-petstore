# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies to cache them
RUN cargo build --release && rm -rf src

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create a minimal image with the binary
FROM scratch

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-server-petstore /rust-server-petstore

# Expose the port your application listens on (if any)
EXPOSE 8080
# Set the binary as the entry point
ENTRYPOINT ["/rust-server-petstore"]