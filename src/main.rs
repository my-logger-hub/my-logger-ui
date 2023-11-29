use std::sync::Arc;

use crate::app_ctx::AppContext;
use crate::states::*;
use crate::views::*;

use dioxus::prelude::*;
use dioxus_liveview::LiveViewPool;

use salvo::affix;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

mod app_ctx;
mod grpc_client;
mod http_server;
mod log_event_context_parser;
mod settings_reader;
mod states;
mod static_resources;
mod views;

lazy_static::lazy_static! {
    pub static ref APP_CTX: Arc<AppContext> = {
        Arc::new(AppContext::new())
    };
}

#[allow(non_snake_case)]
pub mod my_logger_grpc {
    tonic::include_proto!("my_logger");
}

#[tokio::main]
async fn main() {
    let settings = crate::settings_reader::SettingsReader::new(".my-logger-ui").await;

    APP_CTX.apply_settings(Arc::new(settings)).await;

    let acceptor = TcpListener::new("0.0.0.0:9001").bind().await;
    let view = LiveViewPool::new();

    let router = Router::new()
        .hoop(affix::inject(Arc::new(view)))
        .get(http_server::index)
        .push(Router::with_path("ws").get(http_server::connect))
        .push(Router::with_path("img/<**path>").get(StaticDir::new("./files/img")));

    Server::new(acceptor).serve(router).await;
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || LeftMenuState::new());

    let main_panel = match *use_shared_state(cx).unwrap().read() {
        LeftMenuState::Dashboard => rsx! { render_dashboard {} },
        LeftMenuState::Logs => rsx! { render_logs {} },
    };

    render! {
        left_panel {}
        div { id: "main-panel", main_panel }
    }
}
