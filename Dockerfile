# Use the official Rust image as the base image for building
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo files for dependency resolution
COPY Cargo.toml ./

# Build the dependencies separately to cache them
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Remove the dummy source file
RUN rm -rf src

# Copy the source code into the container
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Use a smaller base image for the final release image
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the built executable from the previous stage
COPY --from=builder /usr/src/app/target/release/rust-server-petstore .

# Expose the port your application listens on (if any)
EXPOSE 8080

# Command to run the executable
CMD ["./rust-server-petstore"]
