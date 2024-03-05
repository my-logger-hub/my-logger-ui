FROM myjettools/dioxus-alpha:0.1.0
COPY . .
RUN dx build --platform fullstack --release
ENTRYPOINT ["dx", "serve", "--playform", "fullstack", "--release" ]