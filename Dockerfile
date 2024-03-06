FROM ubuntu:22.04
COPY ./target/release/my-logger-ui ./target/release/my-logger-ui
COPY ./dist ./target/release/dist
RUN chmod +x ./target/release/my-logger-ui
ENTRYPOINT ["./target/release/my-logger-ui" ]