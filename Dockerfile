FROM rust:1.85-bookworm AS build

WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --locked --release --target wasm32-unknown-unknown
# The CLI must match the wasm-bindgen version resolved by Cargo.lock.
RUN cargo install wasm-bindgen-cli --version 0.2.93 --locked
COPY out ./out
RUN wasm-bindgen --no-typescript --target web --out-dir ./out --out-name CrabGame \
    ./target/wasm32-unknown-unknown/release/crab_game.wasm
COPY assets ./out/assets

FROM nginx:alpine

COPY --from=build /app/out /usr/share/nginx/html

EXPOSE 80
