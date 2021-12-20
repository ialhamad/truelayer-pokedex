
############### Builder stage ###############

# We use the latest Rust stable release as base image
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --bin pokedex

############### Runtime stage ###############

FROM debian:buster-slim AS runtime

WORKDIR /app

# Lauch our binary
ENTRYPOINT ["./pokedex"]

# Copy the compiled binary from the builder environment
COPY --from=builder /app/target/release/pokedex pokedex
