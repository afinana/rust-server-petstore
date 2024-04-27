FROM rust:1.77-alpine AS builder
RUN apk add musl-dev --no-cache
WORKDIR /src
COPY . .
RUN cargo build --release

FROM alpine:3.15
WORKDIR /app
COPY --from=builder /src/target/release/rust-server-petstore .

# Expose the port your application listens on (if any)
EXPOSE 8080
ENTRYPOINT [ "./rust-server-petstore" ]
