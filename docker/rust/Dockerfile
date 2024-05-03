FROM rust:1.78.0-alpine3.19 AS builder

WORKDIR /usr/src/dashboard
RUN apk upgrade
RUN apk add --no-cache musl-dev curl

# Install cargo-leptos
RUN curl --proto '=https' --tlsv1.2 -LsSf https://leptos-rs.artifacts.axodotdev.host/cargo-leptos/v0.2.17/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo-leptos build --release -vv

FROM alpine:3.19.1
RUN apk update

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/denux-dashboard /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/app/denux-dashboard"]