FROM ubuntu:22.04
COPY ./target/release/my-logger-ui ./target/release/my-logger-ui
COPY ./files ./files
ENTRYPOINT ["dx", "serve", "--playform", "fullstack", "--release" ]