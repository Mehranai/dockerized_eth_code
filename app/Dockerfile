# ---------- Build stage ----------
FROM rust:1.75 AS builder

WORKDIR /app

# برای cache بهتر dependency ها
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# سورس واقعی
COPY . .
RUN cargo build --release


# ---------- Runtime stage ----------
FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# اسم باینری از Cargo.toml
COPY --from=builder /app/target/release/arz_axum_for_eth /app/app

EXPOSE 3000

CMD ["./app"]