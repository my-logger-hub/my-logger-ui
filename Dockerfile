FROM myjettools/dioxus-docker:0.7.0

ENV PORT=9001
ENV IP=0.0.0.0

EXPOSE 9001

COPY ./target/dx/my-logger-ui/release/web /target/dx/my-logger-ui/release/web
RUN chmod +x /target/dx/my-logger-ui/release/web/my-logger-ui
WORKDIR /target/dx/my-logger-ui/release/web/
ENTRYPOINT ["./my-logger-ui" ]