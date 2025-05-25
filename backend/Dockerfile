# Build
FROM rust:1.86 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -r src

COPY . .
RUN cargo build --release

# Run
FROM debian:trixie-slim
WORKDIR /app

RUN apt-get update
RUN apt-get install -y docker.io
RUN apt-get install -y docker-compose

COPY --from=builder /app/target/release/backend .

EXPOSE 8081

CMD ["./backend"]
