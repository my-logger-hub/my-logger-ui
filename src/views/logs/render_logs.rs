use std::rc::Rc;

use dioxus::prelude::*;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::{states::*, storage_settings::log_level::SelectedLevel};

use crate::models::*;

use super::*;

use crate::components::*;

#[component]
pub fn RenderLogs() -> Element {
    use_context_provider(|| Signal::new(SearchPanelState::new()));

    let search_panel_state = consume_context::<Signal<SearchPanelState>>();

    let mut main_state = consume_context::<Signal<MainState>>();

    let (logs_data, time_zone, env) = {
        let main_state = main_state.read();
        (
            main_state.logs_data.clone(),
            main_state.get_selected_timezone(),
            main_state.get_selected_env(),
        )
    };

    let is_ctx_search = {
        let search_panel_state = consume_context::<Signal<SearchPanelState>>();
        let search_panel_state_read_access = search_panel_state.read();
        search_panel_state_read_access.search_type.is_ctx_search()
    };

    let cursor = if is_ctx_search { "cursor:pointer;" } else { "" };

    let env_on_refresh = env.clone();
    let top_panel = rsx! {
        RenderLogsPanel {
            env: env.clone(),
            time_zone,
            on_refresh_click: EventHandler::new(move |v| { load_log_items(env_on_refresh.clone(), v) })
        }
    };

    let items = match logs_data {
        DataState::None => {
            main_state.write().logs_data = DataState::Loading;
            load_log_items(env.clone(), search_panel_state.read().clone());
            return rsx! {
                {top_panel},
                {loading_panel()}
            };
        }
        DataState::Loading => {
            return rsx! {
                {top_panel},
                {loading_panel()}
            };
        }
        DataState::Loaded(value) => value,
        DataState::Error(err) => return rsx! { "Error loading data: {err}" },
    };

    let items = items.iter().map(|itm| {
        let itm = itm.clone();

        let dt = DateTimeAsMicroseconds::new(itm.timestamp);
        let dt = time_zone.to_local_time(dt);

        let key_values: Vec<_> = itm
            .ctx
            .iter()
            .map(|ctx| {
                let key = ctx.key.to_string();
                let value = ctx.value.to_string();
                rsx! {
                    div {
                        style: "margin:0;padding:0; {cursor}",
                        onclick: move |_| {
                            if is_ctx_search {
                                consume_context::<Signal<SearchPanelState>>()
                                    .write()
                                    .append_filter_conditions(&key, &value);
                            }
                        },
                        "{key}: '{value}'"
                    }
                }
            })
            .collect();
        let log_ball = render_log_ball(itm.level.clone());
        let process_name = itm.process_name.clone();
        rsx! {
            tr { style: "border-top: 1px solid lightgray;",
                td { {log_ball} }
                td { style: "margin:0;padding:0", "{&dt.to_rfc3339()[..26]}" }
                td { style: "margin:0;padding:0",
                    div {
                        style: "{cursor}",
                        onclick: move |_| {
                            if is_ctx_search {
                                consume_context::<Signal<SearchPanelState>>()
                                    .write()
                                    .append_filter_conditions("Process", &process_name);
                            }
                        },
                        "{itm.process_name}"
                    }
                }
                td { style: "margin:0;padding:0", "{itm.message}" }
                td { {key_values.into_iter()} }
            }
        }
    });

    rsx! {
        {top_panel},
        table { class: "table table-striped", style: "margin-top: 61px;",
            tr {
                th { style: "width: 24px;" }
                th { "Time" }
                th { "Process" }
                th { "Message" }
                th { "Context" }
            }
            {items}
        }
    }
}

fn loading_panel() -> Element {
    rsx! {
        h1 { style: "margin-top:50px;", "Loading..." }
    }
}

fn load_log_items(env: Rc<String>, search_panel_state: SearchPanelState) {
    spawn(async move {
        let main_state = consume_context::<Signal<MainState>>();
        match search_panel_state.search_type {
            SearchType::Ctx => {
                load(
                    env.clone(),
                    &search_panel_state.time_range,
                    main_state,
                    crate::log_event_context_parser::parse_key_value_from_string(
                        search_panel_state.filter.as_str(),
                    ),
                );
            }

            SearchType::Text => {
                search_as_text(
                    main_state,
                    env.clone(),
                    &search_panel_state.time_range,
                    search_panel_state.filter,
                );
            }
        }
    });
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LogApiLevel {
    Info,
    Warning,
    Error,
    FatalError,
    Debug,
}

impl LogApiLevel {
    pub const ALL_LEVELS: [LogApiLevel; 5] = [
        LogApiLevel::Info,
        LogApiLevel::Warning,
        LogApiLevel::Error,
        LogApiLevel::FatalError,
        LogApiLevel::Debug,
    ];
    pub fn as_str(&self) -> &str {
        match self {
            LogApiLevel::Info => "Info",
            LogApiLevel::Warning => "Warning",
            LogApiLevel::Error => "Error",
            LogApiLevel::FatalError => "Fatal",
            LogApiLevel::Debug => "Debug",
        }
    }

    pub fn from_str(src: &str) -> LogApiLevel {
        match src {
            "Info" => LogApiLevel::Info,
            "Warning" => LogApiLevel::Warning,
            "Error" => LogApiLevel::Error,
            "Fatal" => LogApiLevel::FatalError,
            "Debug" => LogApiLevel::Debug,
            _ => LogApiLevel::Info,
        }
    }
}

#[cfg(feature = "server")]
impl Into<LogApiLevel> for crate::server::my_logger_grpc::LogLevelGrpcModel {
    fn into(self) -> LogApiLevel {
        use crate::server::my_logger_grpc::*;
        match self {
            LogLevelGrpcModel::Info => LogApiLevel::Info,
            LogLevelGrpcModel::Warning => LogApiLevel::Warning,
            LogLevelGrpcModel::Error => LogApiLevel::Error,
            LogLevelGrpcModel::Fatal => LogApiLevel::FatalError,
            LogLevelGrpcModel::Debug => LogApiLevel::Debug,
        }
    }
}

#[cfg(feature = "server")]
impl Into<crate::server::my_logger_grpc::LogLevelGrpcModel> for LogApiLevel {
    fn into(self) -> crate::server::my_logger_grpc::LogLevelGrpcModel {
        use crate::server::my_logger_grpc::*;
        match self {
            LogApiLevel::Info => LogLevelGrpcModel::Info,
            LogApiLevel::Warning => LogLevelGrpcModel::Warning,
            LogApiLevel::Error => LogLevelGrpcModel::Error,
            LogApiLevel::FatalError => LogLevelGrpcModel::Fatal,
            LogApiLevel::Debug => LogLevelGrpcModel::Debug,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogApiItem {
    pub timestamp: i64,
    pub tenant_id: String,
    pub process_name: String,
    pub message: String,
    pub level: LogApiLevel,
    pub ctx: Vec<LogEventContextApiModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEventContextApiModel {
    pub key: String,
    pub value: String,
}

fn load<'s>(
    env: Rc<String>,
    time_range: &TimeRange,
    mut main_state: Signal<MainState>,
    context_keys: Vec<LogEventContextApiModel>,
) {
    {
        main_state.write().logs_data = DataState::Loading;
    }

    let log_level_filter = crate::storage_settings::log_level::get();
    let level = match log_level_filter {
        SelectedLevel::All => None,
        SelectedLevel::Info => Some(LogApiLevel::Info),
        SelectedLevel::Warning => Some(LogApiLevel::Warning),
        SelectedLevel::Error => Some(LogApiLevel::Error),
        SelectedLevel::FatalError => Some(LogApiLevel::FatalError),
        SelectedLevel::Debug => Some(LogApiLevel::Debug),
    };

    let context_keys = if context_keys.is_empty() {
        None
    } else {
        Some(context_keys)
    };

    let (from, to) = time_range.get_date_from_date_to();
    spawn(async move {
        let result = load_logs(env.to_string(), level, from, to, context_keys).await;

        match result {
            Ok(result) => {
                let items: Vec<Rc<LogApiItem>> = result.into_iter().map(Rc::new).collect();
                main_state.write().logs_data = DataState::Loaded(items);
            }

            Err(err) => {
                main_state.write().logs_data = DataState::Error(err.to_string());
            }
        }
    });
}

pub fn search_as_text(
    mut main_state: Signal<MainState>,
    env: Rc<String>,
    time_range: &TimeRange,
    phrase: String,
) {
    let (from, to) = time_range.get_date_from_date_to();
    spawn(async move {
        let result = search_logs(env.to_string(), from, to, phrase).await;

        match result {
            Ok(result) => {
                main_state.write().logs_data =
                    DataState::Loaded(result.into_iter().map(Rc::new).collect());
            }
            Err(err) => {
                main_state.write().logs_data = DataState::Error(err.to_string());
            }
        }
    });
}

#[server]
pub async fn search_logs(
    env: String,
    from_time: i64,
    to_time: i64,
    phrase: String,
) -> Result<Vec<LogApiItem>, ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let ui_url = crate::server::APP_CTX.get_ui_url(&env).await;
    let result = crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .scan_and_search(ScanAndSearchRequest {
            ui_url,
            from_time: from_time,
            to_time: to_time,
            take: 200,
            phrase,
        })
        .await
        .unwrap();

    let result = match result {
        Some(result) => result
            .into_iter()
            .map(|itm| {
                let level: LogApiLevel = (&itm.level()).into();

                LogApiItem {
                    timestamp: itm.timestamp,
                    tenant_id: itm.tenant_id,
                    process_name: itm.process_name,
                    message: itm.message,
                    level,
                    ctx: itm
                        .ctx
                        .into_iter()
                        .map(|itm| LogEventContextApiModel {
                            key: itm.key,
                            value: itm.value,
                        })
                        .collect(),
                }
            })
            .collect(),
        None => vec![],
    };

    Ok(result)
}

#[server]
pub async fn load_logs(
    env: String,
    level: Option<LogApiLevel>,
    from_time: i64,
    to_time: i64,
    ctx: Option<Vec<LogEventContextApiModel>>,
) -> Result<Vec<LogApiItem>, ServerFnError> {
    use crate::server::my_logger_grpc::*;
    println!("Load logs '{}'-'{}'", from_time, to_time);

    let levels = if let Some(level) = level {
        match level {
            LogApiLevel::Info => vec![LogLevelGrpcModel::Info as i32],
            LogApiLevel::Warning => vec![LogLevelGrpcModel::Warning as i32],
            LogApiLevel::Error => vec![LogLevelGrpcModel::Error as i32],
            LogApiLevel::FatalError => vec![LogLevelGrpcModel::Fatal as i32],
            LogApiLevel::Debug => vec![LogLevelGrpcModel::Debug as i32],
        }
    } else {
        vec![]
    };

    let ctx = ctx.unwrap_or_default();

    let result = crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .read(ReadLogEventRequest {
            ui_url: crate::server::APP_CTX.get_ui_url(&env).await,
            from_time: from_time,
            to_time: to_time,
            levels,
            context_keys: ctx
                .into_iter()
                .map(|itm| LogEventContext {
                    key: itm.key,
                    value: itm.value,
                })
                .collect(),
            take: 200,
            skip: 0,
        })
        .await
        .unwrap();

    let result = match result {
        Some(result) => result
            .into_iter()
            .map(|itm| {
                let level: LogApiLevel = (&itm.level()).into();

                LogApiItem {
                    timestamp: itm.timestamp,
                    tenant_id: itm.tenant_id,
                    process_name: itm.process_name,
                    message: itm.message,
                    level,
                    ctx: itm
                        .ctx
                        .into_iter()
                        .map(|itm| LogEventContextApiModel {
                            key: itm.key,
                            value: itm.value,
                        })
                        .collect(),
                }
            })
            .collect(),
        None => vec![],
    };

    Ok(result)
}
