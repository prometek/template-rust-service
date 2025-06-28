FROM rust:1.70-slim
WORKDIR /app

# Build minimal binary
COPY Cargo.toml Config.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy source and rebuild
COPY . .
RUN cargo build --release

# Runtime image
FROM debian:buster-slim
WORKDIR /app
COPY --from=0 /app/target/release/rust-template-service ./
COPY Config.toml ./
EXPOSE 8000
CMD ["./template-rust-service"]
