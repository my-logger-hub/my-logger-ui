FROM rust
RUN apt-get update && apt-get install -y pkg-config libssl-dev perl make \
    && cargo install dioxus-cli@0.5.0-alpha.0 && rustup target add wasm32-unknown-unknown
COPY ./target ./target
COPY ./files ./files
COPY ./dist ./dist
COPY ./.dioxus ./.dioxus
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./Dioxus.toml ./Dioxus.toml
ENTRYPOINT ["dx", "serve", "--playform", "fullstack", "--release" ]