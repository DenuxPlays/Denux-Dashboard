FROM rust:1.74.0-alpine3.18 as builder

WORKDIR /usr/src/dashboard
RUN apk upgrade
RUN apk add --no-cache musl-dev libressl-dev

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM alpine:3.18.4
RUN apk update

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/denux_dashboard /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
WORKDIR /app

ENTRYPOINT ["/app/denux_dashboard"]