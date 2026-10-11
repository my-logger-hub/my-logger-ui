# my-logger-ui

UI for [my-logger-server](https://github.com/my-logger-hub/my-logger-server): an HTTP server which
serves the Dioxus client-side (WASM) UI and the `/api/...` it calls. The server talks to MyLogger
server of every environment over gRPC (directly or through an ssh tunnel).

## Layout

| Folder | What it is |
| --- | --- |
| `src/` | The server (`my-http-server`): HTTP actions in `src/http_server`, gRPC client of MyLogger server in `src/grpc_client` |
| `ui/` | The UI - a Dioxus client-side (WASM) app, calls the server with FlUrl |
| `rest-api-shared/` | Request / response models shared verbatim by the server and the UI |
| `wwwroot/` | The built UI. Committed: the Dockerfile bakes it into the image |
| `proto/` | `MyLogger.proto` - a copy of `my-logger-server/proto/MyLogger.proto` |

## Build

```bash
# the server - the API and the UI at port 8000. CI builds it on every release tag
cargo build --release

# the UI - on the Mac mini, after any change under ui/ or rest-api-shared/; commit wwwroot/ afterwards
./build-ui.sh
```

CI does not build the UI: the image gets `wwwroot/` exactly as the last `./build-ui.sh` left it.
The UI is built with the same optimisations as the trading terminals (mt-client,
my-prop-trading-client):

- `[profile.release]` in `ui/Cargo.toml`: `opt-level = 3`, fat LTO, one codegen unit, `panic = "abort"`;
- full `wasm-opt -Oz` (`[web.wasm_opt]` in `ui/Dioxus.toml`);
- `--debug-symbols false` - no names and no debug info in the wasm (instead of `strip`, which
  breaks the wasm-bindgen section on Rust 1.99).

The script checks that `dx` is the pinned 0.7.10, stops if wasm-opt did not run or failed (dx
itself would ship the module unoptimised), takes the stylesheet build.rs regenerated, and
removes `ui/target` (gigabytes) once the bundle is staged.

## Development

```bash
# terminal 1 - the server at http://localhost:8000
cargo run

# terminal 2 - the UI with hot reload at http://localhost:8080; /api/... is proxied to :8000
cd ui && dx serve
```

Swagger of the API: http://localhost:8000/swagger

## Settings

`~/.my-logger` - read at startup: without it the server does not start.

```yaml
envs:
  env-1: ssh:gateway@10.0.0.0:22->http://10.0.0.3:8888
  env-2: ssh:gateway@10.0.0.1:22->http://10.0.0.3:8888
```

```yaml
envs:
  env-1: ssh:gateway@10.0.0.0:22->http://10.0.0.3:8888
  env-2: ssh:gateway@10.0.0.1:22->http://10.0.0.3:8888

ssh_private_keys:
  "*":
    cert_path: /root/cert
    cert_pass_phrase: password
```

```yaml
envs:
  env-1: ssh:gateway@10.0.0.0:22->http://10.0.0.3:8888
  env-2: ssh:gateway@10.0.0.1:22->http://10.0.0.3:8888

ssh_private_keys:
  "gateway@10.0.0.0:22":
    cert_path: /root/cert-1
    cert_pass_phrase: password

  "gateway@10.0.0.1:22":
    cert_path: /root/cert-2
    cert_pass_phrase: password
```

ssh_private_keys - can be missing. In this case SshAgent will be used.
