use std::rc::Rc;

use dioxus::prelude::*;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::{main_state::MainState, views::render_log_ball};

#[derive(Debug, Clone, Copy)]
pub enum SelectedLevel {
    All,
    Info,
    Warning,
    Error,
    FatalError,
    Debug,
}
#[derive(Debug, Clone, Copy)]
pub enum SearchType {
    Ctx,
    Text,
}

#[derive(Clone, Copy)]
pub struct HoursAgo(i64);

impl HoursAgo {
    pub fn get_value(&self) -> i64 {
        self.0
    }
}

#[component]
pub fn RenderLogs() -> Element {
    let main_state = consume_context::<Signal<MainState>>();
    let mut search_type = use_signal(|| SearchType::Ctx);
    let mut log_level_filter: Signal<SelectedLevel, _> = use_signal(|| SelectedLevel::All);
    let mut hours_ago_filter: Signal<HoursAgo, _> = use_signal(|| HoursAgo(2));

    let mut ctx_filter: Signal<String, _> = use_signal(|| "".to_string());

    let logs_data = main_state.read().logs_data.clone();

    let log_level_value_as_str = format!("{:?}", log_level_filter.clone());

    let hours_ago_value = hours_ago_filter.read().clone();

    let ctx_filter_value = Rc::new(ctx_filter.read().clone());

    let ctx_filter_panel_value = ctx_filter_value.clone();

    let main_state = consume_context::<Signal<MainState>>();

    let env = main_state.read().selected_env.clone();
    let env_on_click = env.clone();

    let search_placeholder = match &*search_type.read() {
        SearchType::Ctx => "Example: Application='MyApp' ; Version='Version'",
        SearchType::Text => "Just a text",
    };

    let panel = rsx! {

        table { style: "width: calc(var(--app-width) - var(--panel-width)); border-bottom: 1px lightgray solid; box-shadow: 0 0 5px lightgray; position: fixed; background:white",
            tr {
                td {
                    div { class: "input-group input-group-sm",
                        span { class: "input-group-text", "Level:" }

                        select {

                            class: "form-control",

                            onchange: move |e| {
                                match e.value().as_str() {
                                    "Info" => log_level_filter.set(SelectedLevel::Info),
                                    "Warning" => log_level_filter.set(SelectedLevel::Warning),
                                    "Error" => log_level_filter.set(SelectedLevel::Error),
                                    "FatalError" => log_level_filter.set(SelectedLevel::FatalError),
                                    "Debug" => log_level_filter.set(SelectedLevel::Debug),
                                    _ => log_level_filter.set(SelectedLevel::All),
                                }
                            },
                            value: "{log_level_value_as_str}",
                            option { "All" }
                            option { "Info" }
                            option { "Warning" }
                            option { "Error" }
                            option { "FatalError" }
                            option { "Debug" }
                        }
                    }
                }
                td {
                    div { class: "input-group input-group-sm",
                        span { class: "input-group-text", "From hours ago:" }
                        input {
                            style: "width: 70px",
                            class: "form-control",
                            r#type: "number",
                            value: "{hours_ago_value.get_value()}",
                            oninput: move |e| {
                                if let Ok(value) = e.value().parse::<i64>() {
                                    hours_ago_filter.set(HoursAgo(value));
                                }
                            }
                        }
                    }
                }
                td { style: "padding-right: 0px; padding-left: 10px;",

                    select {
                        style: "width: 100%;border: 1px solid lightgray;border-radius: 5px 0 0 5px;border-right: none;background-color: var(--vz-tertiary-bg);",
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
                td { style: "width:60%;padding-left: 0px;",
                    input {
                        class: "form-control form-control-sm",
                        placeholder: search_placeholder,
                        value: "{ctx_filter_value}",
                        oninput: move |e| {
                            ctx_filter.set(e.value());
                        }
                    }
                }
                td {
                    button {
                        class: "btn btn-primary btn-sm",

                        onclick: move |_| {
                            let mut main_state = consume_context::<Signal<MainState>>();
                            main_state.write().set_logs_data(None);
                            match search_type.read().clone() {
                                SearchType::Ctx => {
                                    load(
                                        env_on_click.clone(),
                                        log_level_filter.read().clone(),
                                        hours_ago_filter.read().get_value(),
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
                                        hours_ago_filter.read().get_value(),
                                        ctx_filter_panel_value.to_string(),
                                    );
                                }
                            }
                        },
                        "Get data"
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
                    log_level_filter.read().clone(),
                    hours_ago_filter.read().get_value(),
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
                    hours_ago_filter.read().get_value(),
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

        let dt = DateTimeAsMicroseconds::new(itm.timestamp);
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
        table { class: "table table-striped", style: "margin-top:36px",
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
    log_level_filter: SelectedLevel,
    hours_before: i64,
    mut main_state: Signal<MainState>,
    context_keys: Vec<LogEventContextApiModel>,
) {
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

    spawn(async move {
        let result = load_logs(env.to_string(), level, hours_before, context_keys)
            .await
            .unwrap();

        main_state.write().set_logs_data(Some(result));
    });
}

pub fn search_as_text(
    mut main_state: Signal<MainState>,
    env: Rc<String>,
    hours_before: i64,
    phrase: String,
) {
    spawn(async move {
        let result = search_logs(env.to_string(), hours_before, phrase)
            .await
            .unwrap();

        main_state.write().set_logs_data(Some(result));
    });
}

#[server]
pub async fn search_logs(
    env: String,
    hours_before: i64,
    phrase: String,
) -> Result<Vec<LogApiItem>, ServerFnError> {
    use crate::my_logger_grpc::*;
    let mut from_time = DateTimeAsMicroseconds::now();
    let to_time = from_time.clone();
    from_time.add_hours(-hours_before);

    let result = crate::APP_CTX
        .get_client(env.as_str())
        .await
        .scan_and_search(ScanAndSearchRequest {
            tenant_id: "Default".to_string(),
            from_time: from_time.unix_microseconds as u64,
            to_time: to_time.unix_microseconds as u64,
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
    hours_before: i64,
    ctx: Option<Vec<LogEventContextApiModel>>,
) -> Result<Vec<LogApiItem>, ServerFnError> {
    use crate::my_logger_grpc::*;

    let mut from_time = DateTimeAsMicroseconds::now();

    from_time.add_hours(-hours_before);

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
            tenant_id: "Default".to_string(),
            from_time: from_time.unix_microseconds,
            to_time: 0,
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

/*
fn load<'s>(
    cx: &'s Scope<'s>,
    log_level_filter: SelectedLevel,
    hours_before: i64,

    logs_state: &UseState<Option<Vec<Rc<LogEventGrpcModel>>>>,
    context_keys: Vec<LogEventContext>,
) {
    let logs_state = logs_state.to_owned();

    println!("{:?}", context_keys);

    cx.spawn(async move {
        let grpc_client = APP_CTX.get_my_logger_grpc_client().await;

        let mut from_time = DateTimeAsMicroseconds::now();

        from_time.add_hours(-hours_before);

        let levels = match log_level_filter {
            SelectedLevel::All => vec![],
            SelectedLevel::Info => vec![LogLevelGrpcModel::Info as i32],
            SelectedLevel::Warning => vec![LogLevelGrpcModel::Warning as i32],
            SelectedLevel::Error => vec![LogLevelGrpcModel::Error as i32],
            SelectedLevel::FatalError => vec![LogLevelGrpcModel::Fatal as i32],
            SelectedLevel::Debug => vec![LogLevelGrpcModel::Debug as i32],
        };

        let a = grpc_client
            .read(ReadLogEventRequest {
                tenant_id: "Default".to_string(),
                from_time: from_time.unix_microseconds,
                to_time: 0,
                levels,
                context_keys,
                take: 200,
                skip: 0,
            })
            .await
            .unwrap();

        let a = match a {
            Some(a) => a.into_iter().map(|itm| Rc::new(itm)).collect(),
            None => vec![],
        };

        logs_state.set(Some(a));
    });
}
 */
