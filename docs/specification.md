# my-logger-ui — specification

How the UI works inside. Setting it up and running it: [README.md](../README.md).

## Layout

| Folder | What it is |
| --- | --- |
| `src/` | The server (`my-http-server`): HTTP actions in `src/http_server`, the flows behind them in `src/flows`, the gRPC client of MyLogger server in `src/grpc_client` |
| `ui/` | The UI — a Dioxus client-side (WASM) app; calls the server with FlUrl |
| `rest-api-shared/` | Request / response models shared verbatim by the server (with the `server` feature) and the UI (without it) |
| `wwwroot/` | The built UI. Committed: the Dockerfile bakes it into the image |
| `proto/` | `MyLogger.proto` — a copy of `my-logger-server/proto/MyLogger.proto` (see [Proto](#proto)) |

## How a request goes

Browser (WASM) → `/api/...` of this server, port 8000 → gRPC to MyLogger server of the env.

- The envs and their urls are in the settings (`envs`). A url is either `http://host:port` or one through an ssh tunnel, `ssh:user@ssh-host:22->http://host:port`.
- A gRPC client per env is created with the first request to the env and reused after it.
- An ssh tunnel is authenticated with the key `ssh_private_keys` gives for `user@ssh-host:port` (or for `*`), and through the ssh agent when there is none.
- The UI sends the origin it is opened at to `GET /api/app/v1/info`. When it is https, the server passes `<origin>/dashboard/<env>` to MyLogger server with every logs request (`UiUrl`): MyLogger server builds links back to the UI from it — in the Telegram messages, for example.
- The same port serves the UI from `wwwroot/`: a path which is not a file gets `index.html`, so the routes of the SPA open directly.

The UI and MyLogger server have versions of their own, and no backward compatibility is kept between them.

## HTTP API

Swagger: `/swagger`. Every endpoint but `app/v1/info` takes the env (`env`) and answers 404 for an env which is not in the settings.

| Endpoint | What it does |
| --- | --- |
| `GET /api/app/v1/info?uiUrl=` | The version of the app and the envs of the settings |
| `GET /api/server/v1/info?env=` | The version and `hours_to_gc` of MyLogger server of the env; an empty version — the server is not reachable |
| `GET /api/dashboard/v1/hourly-statistics?env=` | The counters per hour, product scope, application and level for the last 8 hours (`GetHourlyStatistics`) |
| `POST /api/logs/v1/load` | Logs of the time range by level and context key-values (`Read`), at most 200 |
| `POST /api/logs/v1/search` | Logs of the time range which contain the phrase (`ScanAndSearch`), at most 200 |
| `GET /api/product-scopes/v1/list?env=` | The product scopes of the env (`GetProductScopes`) |
| `GET /api/insights/v1/keys?env=` | The context keys MyLogger server has insights for |
| `GET /api/ignore-events/v1/list?env=`, `POST .../add`, `POST .../delete` | The ignore events |
| `GET /api/one-time-ignore-events/v1/list?env=`, `POST .../save`, `POST .../delete` | The one time ignore events |

`fromTime` / `toTime` of `load` and `search` are passed to MyLogger server as they are, encoded by the time-range contract of [my-logger-server](https://github.com/my-logger-hub/my-logger-server/blob/main/docs/specification.md#time-range-contract).

## Product scopes

MyLogger server keeps and searches the logs by product scope; its specification says how a record gets its scope.

**Logs.** A load or a search reads one scope: `productScope` of the request, the default scope of the env when it is absent or empty. The scope to read is chosen next to the level:

- the choice is shown when the env has more than one scope, or when the scope read is not the default one — a link may name a scope the env does not list, and it is shown as it is;
- the default scope is kept as none, so the env decides which one it is;
- the scope is a part of the logs url and is kept in the local storage (`product-scope`); changing the env clears it.

**Dashboard.** The statistics are of every scope. A row is an application of a scope: an application whose logs went to several scopes has a row per scope, and its scope is shown under its name when the hours shown have more than one scope. The Errors / Fatal errors / Warnings links open the logs of the application in its scope. The rows of an hour go by the errors and the fatal errors, the most first.

## Logs url

`/logs/<base64>` — a protobuf message (`ui/src/models/log_path_data_model.rs`), so a link opens the same search:

| Tag | Field | |
| --- | --- | --- |
| 1 | `level` | 0 Error, 1 FatalError, 2 Warning, 3 Info, 4 Debug, 100 all |
| 2 | `is_ctx_search` | Ctx search or text search |
| 3 | `time_range` | As the time-range field shows it |
| 4 | `search_string` | The search line |
| 5 | `product_scope` | Empty — the default scope |

Opening such a url writes its fields to the local storage (`level`, `ctx-search`, `time-range`, `search-line`, `product-scope`); the logs page reads the search from there. The local storage also keeps `env`, `theme` and `selected-timezone`.

## Proto

`proto/MyLogger.proto` is a copy of the one of my-logger-server, compiled from the local file by `build.rs`: downloading it with ci-utils brings flurl → my-http-utils into the build dependencies, and that copy breaks the `server` feature my-http-server needs from my-http-utils.

`SearchUp` and `SearchDown` are left out of the copy: both are `SearchLogsRequest → stream LogEventGrpcModel`, and `generate_grpc_client` of my-grpc-extensions implements its executor traits by the request and response types, so the second one conflicts with the first (E0119).

## UI build

CI builds the server only; `wwwroot/` gets into the image exactly as the last `./build-ui.sh` (`ui/build.sh`) left it. The bundle is built on the Mac mini: full `wasm-opt -Oz` on a big module takes more RAM than a CI runner has.

What makes the bundle small — the same recipe as the trading terminals (mt-client, my-prop-trading-client):

- `[profile.release]` in `ui/Cargo.toml`: `opt-level = 3`, fat LTO, one codegen unit, `panic = "abort"`;
- full `wasm-opt -Oz` (`[web.wasm_opt]` in `ui/Dioxus.toml`);
- `--debug-symbols false` — no names and no debug info in the wasm, instead of `strip`, which breaks the wasm-bindgen section on Rust 1.99.

The script:

- stops unless `dx` is the pinned 0.7.10 — it must match the dioxus version of `ui/Cargo.toml`;
- stops when wasm-opt did not run or failed: dx itself ships the module unoptimised then;
- takes the stylesheet `build.rs` has just regenerated from `ui/css/` — dx collects the assets before it and would ship the previous one;
- replaces `wwwroot/` and removes `ui/target` (gigabytes) once the bundle is staged.

In development `dx serve` (port 8080) forwards `/api/...` to the server at port 8000 (`[[web.proxy]]` in `ui/Dioxus.toml`).
