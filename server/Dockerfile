# Build stage
FROM rust:latest as builder
COPY . /app/
WORKDIR /app
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/journly-server .
CMD ["./journly-server"]
