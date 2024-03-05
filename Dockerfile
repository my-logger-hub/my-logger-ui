FROM dioxus-alpha:0.1.0
COPY ./target ./target
COPY ./files ./files
COPY ./dist ./dist
COPY ./.dioxus ./.dioxus
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./Dioxus.toml ./Dioxus.toml
ENTRYPOINT ["dx", "serve", "--playform", "fullstack", "--release" ]