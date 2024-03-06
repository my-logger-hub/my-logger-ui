FROM ubuntu:22.04
COPY ./target/release/my-logger-ui ./target/release/my-logger-ui
COPY ./dist ./target/release/dist
ENTRYPOINT ["my-logger-ui" ]