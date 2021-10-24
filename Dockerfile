FROM rust:1.56.0 AS builder
WORKDIR /usr/src/bayer-warp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bayer-warp /usr/local/bin/bayer-warp
ENTRYPOINT ["bayer-warp"]
