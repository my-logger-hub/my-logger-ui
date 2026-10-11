FROM ubuntu:22.04
COPY ./target/release/my-logger-ui ./target/release/my-logger-ui
COPY ./wwwroot ./wwwroot
ENTRYPOINT ["./target/release/my-logger-ui"]
