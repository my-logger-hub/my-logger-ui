# my-logger-ui

UI for [my-logger-server](https://github.com/my-logger-hub/my-logger-server): the logs, the hourly statistics and the ignore lists of every environment. How it works inside: [docs/specification.md](docs/specification.md).

## Settings

The YAML file `~/.my-logger`, read at start: without it the server does not start.

```yaml
envs: # the name shown in the UI -> gRPC of MyLogger server of the env
  env-1: http://10.0.0.3:8888
  env-2: ssh:gateway@10.0.0.1:22->http://10.0.0.3:8888 # through an ssh tunnel

ssh_private_keys: # optional: without it the ssh agent is used
  "gateway@10.0.0.1:22": # or "*" - for every ssh host without a key of its own
    cert_path: /root/cert
    cert_pass_phrase: password # optional
```

## Run

The image is published on every release tag; it serves the UI and its API at port 8000:

```bash
docker run -p 8000:8000 -v ~/.my-logger:/root/.my-logger ghcr.io/my-logger-hub/my-logger-ui:<version>
```

Open http://localhost:8000. Swagger of the API: http://localhost:8000/swagger.

## Build

Needs Rust and `protoc` in `PATH`.

```bash
# the server; CI builds it on every release tag
cargo build --release

# the UI - on the Mac mini, after any change under ui/ or rest-api-shared/, then commit wwwroot/
./build-ui.sh
```

CI does not build the UI: the image gets `wwwroot/` as the last `./build-ui.sh` left it. The script needs `dx` 0.7.10:

```bash
cargo binstall dioxus-cli --version 0.7.10 --force
```

## Development

```bash
# terminal 1 - the server at http://localhost:8000
cargo run

# terminal 2 - the UI with hot reload at http://localhost:8080
cd ui && dx serve
```
