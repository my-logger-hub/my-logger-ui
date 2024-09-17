use std::rc::Rc;

use dioxus::prelude::*;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::{
    dialogs::{DialogState, TimeRange},
    states::*,
    storage_settings::log_level::SelectedLevel,
};

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum SearchType {
    Ctx,
    Text,
}

#[component]
pub fn RenderLogs() -> Element {
    let main_state = consume_context::<Signal<MainState>>();
    let mut search_type = use_signal(|| SearchType::Ctx);

    let mut time_range_state = use_signal(|| crate::storage_settings::time_range::get());

    let time_range_value = Rc::new(time_range_state.read().clone());

    let range_label = match time_range_value.as_ref() {
        TimeRange::HoursAgo(_) => "From Hours ago:",
        TimeRange::Range(_, _) => "From Time range:",
        TimeRange::ExactHour(_) => "Get log from Hour:",
    };

    let mut ctx_filter: Signal<String, _> = use_signal(|| {
        let app = dioxus_utils::js::GlobalAppSettings::get_local_storage()
            .get("app")
            .unwrap_or_default();

        if app.len() > 0 {
            return format!("Application='{}'", app);
        }

        "".to_string()
    });

    let (logs_data, time_zone) = {
        let main_state = main_state.read();
        (main_state.logs_data.clone(), main_state.time_zone)
    };

    let ctx_filter_value = Rc::new(ctx_filter.read().clone());

    let ctx_filter_panel_value = ctx_filter_value.clone();

    let main_state = consume_context::<Signal<MainState>>();

    let env = main_state.read().get_selected_env();
    let env_on_click = env.clone();

    let search_placeholder = match &*search_type.read() {
        SearchType::Ctx => "Example: Application='MyApp' ; Version='Version'",
        SearchType::Text => "Just a text",
    };

    let time_range_value_copy = time_range_value.clone();

    let panel = rsx! {

        table { style: "width: calc(var(--app-width) - var(--panel-width)); border-bottom: 1px lightgray solid; box-shadow: 0 0 5px lightgray; position: fixed; background:white",
            tr {
                td { style: "width: 150px;",
                    div { style: "margin-top: 5px;", "Level" }
                    SelectLogLevel {}
                }
                td { style: "width: 260px;",

                    div { style: "margin-top: 5px;", {range_label} }
                    input {
                        style: "width: 100%; cursor: pointer;",
                        class: "form-control form-control-sm",
                        readonly: true,
                        value: time_range_value.to_string(),
                        onclick: move |_| {
                            let value = time_range_state.read().clone();
                            consume_context::<Signal<DialogState>>()
                                .set(DialogState::EditTimeRange {
                                    value,
                                    time_zone,
                                    on_change: EventHandler::new(move |time_range: TimeRange| {
                                        crate::storage_settings::time_range::save(&time_range);
                                        time_range_state.set(time_range);
                                    }),
                                });
                        }
                    }
                }

                td { style: "width:60%;padding-left: 0px;",
                    div {
                        select {
                            style: "width: 100px; border: 1px solid white;",
                            class: "form-select form-select-sm",
                            oninput: move |e| {
                                match e.value().as_str() {
                                    "ctx" => search_type.set(SearchType::Ctx),
                                    "text" => search_type.set(SearchType::Text),
                                    _ => {}
                                }
                            },
                            option { value: "ctx", "Ctx Search" }
                            option { value: "text", "Text Search" }
                        }
                    }
                    input {
                        class: "form-control form-control-sm",
                        placeholder: search_placeholder,
                        value: "{ctx_filter_value}",
                        oninput: move |e| {
                            ctx_filter.set(e.value());
                        }
                    }
                }
                td { style: "width: 32px;vertical-align: bottom;",
                    button {
                        class: "btn btn-primary btn-sm",

                        onclick: move |_| {
                            let mut main_state = consume_context::<Signal<MainState>>();
                            main_state.write().set_logs_data(None);
                            match search_type.read().clone() {
                                SearchType::Ctx => {
                                    load(
                                        env_on_click.clone(),
                                        &time_range_value_copy,
                                        time_zone,
                                        main_state,
                                        crate::log_event_context_parser::parse_key_value_from_string(
                                            ctx_filter_panel_value.as_str(),
                                        ),
                                    );
                                }
                                SearchType::Text => {
                                    search_as_text(
                                        main_state,
                                        env_on_click.clone(),
                                        &time_range_value_copy,
                                        time_zone,
                                        ctx_filter_panel_value.to_string(),
                                    );
                                }
                            }
                        },
                        img {
                            src: "/img/ico-refresh.svg",
                            style: "width: 16px;"
                        }
                    }
                }
            }
        }
    };

    if logs_data.is_none() {
        match search_type.read().clone() {
            SearchType::Ctx => {
                load(
                    env.clone(),
                    &time_range_value,
                    time_zone,
                    main_state,
                    crate::log_event_context_parser::parse_key_value_from_string(
                        ctx_filter_value.as_str(),
                    ),
                );
            }

            SearchType::Text => {
                search_as_text(
                    main_state,
                    env.clone(),
                    &time_range_value,
                    time_zone,
                    ctx_filter_value.to_string(),
                );
            }
        }

        return rsx! {
            {panel},
            h1 { style: "margin-top:50px;", "Loading..." }
        };
    }

    let log_state_value = logs_data.unwrap();

    let items = log_state_value.iter().map(|itm| {
        let itm = itm.clone();

        let mut dt = DateTimeAsMicroseconds::new(itm.timestamp);
        dt.add_minutes(-time_zone);
        let ctx_filter_value = ctx_filter_value.clone();
        let key_values: Vec<_> = itm
            .ctx
            .iter()
            .map(|ctx| {
                let key = ctx.key.to_string();
                let value = ctx.value.to_string();
                let ctx_filter_value = ctx_filter_value.clone();
                rsx! {
                    div {
                        style: "margin:0;padding:0; cursor:pointer;",
                        onclick: move |_| {
                            let mut filter = ctx_filter_value.trim().to_string();
                            append_filter_condition(&mut filter, &key, &value);
                            ctx_filter.set(filter);
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
                        style: "cursor: pointer",
                        onclick: move |_| {
                            let mut filter = ctx_filter_value.trim().to_string();
                            append_filter_condition(&mut filter, "Process", &process_name);
                            ctx_filter.set(filter);
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
        {panel},
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

fn append_filter_condition(filter: &mut String, key: &str, value: &str) {
    if !filter.is_empty() {
        filter.push_str(" and ");
    }
    filter.push_str(key);
    filter.push_str(":'");
    filter.push_str(value);
    filter.push_str("'");
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LogApiLevel {
    Info,
    Warning,
    Error,
    FatalError,
    Debug,
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
    time_zone: i64,
    mut main_state: Signal<MainState>,
    context_keys: Vec<LogEventContextApiModel>,
) {
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

    let (from, to) = time_range.get_date_from_date_to(time_zone);
    spawn(async move {
        let result = load_logs(env.to_string(), level, from, to, context_keys)
            .await
            .unwrap();

        main_state.write().set_logs_data(Some(result));
    });
}

pub fn search_as_text(
    mut main_state: Signal<MainState>,
    env: Rc<String>,
    time_range: &TimeRange,
    time_zone: i64,
    phrase: String,
) {
    let (from, to) = time_range.get_date_from_date_to(time_zone);
    spawn(async move {
        let result = search_logs(env.to_string(), from, to, phrase)
            .await
            .unwrap();

        main_state.write().set_logs_data(Some(result));
    });
}

#[server]
pub async fn search_logs(
    env: String,
    from_time: i64,
    to_time: i64,
    phrase: String,
) -> Result<Vec<LogApiItem>, ServerFnError> {
    use crate::my_logger_grpc::*;

    let result = crate::APP_CTX
        .get_client(env.as_str())
        .await
        .scan_and_search(ScanAndSearchRequest {
            tenant_id: "Default".to_string(),
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
    use crate::my_logger_grpc::*;

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

    let result = crate::APP_CTX
        .get_client(env.as_str())
        .await
        .read(ReadLogEventRequest {
            tenant_id: String::new(),
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
