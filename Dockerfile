FROM ubuntu:22.04
COPY ./target/release/my-logger-ui /target/release/my-logger-ui
COPY ./dist /target/release/dist
RUN chmod +x /target/release/my-logger-ui
WORKDIR /target/release/
ENTRYPOINT ["./my-logger-ui" ]