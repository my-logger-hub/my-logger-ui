use std::rc::Rc;

use dioxus::prelude::*;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{states::*, storage_settings::log_level::SelectedLevel};

use crate::models::*;

use super::*;

use crate::components::*;

use dioxus_utils::*;

#[component]
pub fn RenderLogs() -> Element {
    use_context_provider(|| Signal::new(SearchPanelState::new()));

    let mut search_panel_state = consume_context::<Signal<SearchPanelState>>();

    use_effect(move || {
        if !search_panel_state.peek().matches_storage() {
            search_panel_state.write().load_from_storage();
        }
    });

    let main_state = consume_context::<Signal<MainState>>();
    let main_state_read_access = main_state.read();

    let env = main_state_read_access.get_selected_env();
    let time_zone = main_state_read_access.get_selected_timezone();

    /*
    let (logs_data, time_zone, env) = {
        let main_state = main_state.read();
        (
            main_state.logs_data.clone(),
            main_state.get_selected_timezone(),
            main_state.get_selected_env(),
        )
    };
     */

    let is_ctx_search = {
        let search_panel_state_read_access = search_panel_state.read();
        search_panel_state_read_access.search_type.is_ctx_search()
    };

    let cursor = if is_ctx_search { "cursor:pointer;" } else { "" };

    let env_on_refresh = env.clone();
    let top_panel = rsx! {
        RenderLogsPanel {
            env: env.clone(),
            time_zone,
            on_refresh_click: EventHandler::new(move |v| { load_log_items(env_on_refresh.clone(), v) }),
        }
    };

    let items = match main_state_read_access.logs_data.as_ref() {
        RenderState::None => {
            let sp = search_panel_state.read();
            if sp.initialized {
                load_log_items(env.clone(), sp.clone());
            }
            return rsx! {
                {top_panel}
                {loading_panel()}
            };
        }
        RenderState::Loading => {
            return rsx! {
                {top_panel}
                {loading_panel()}
            };
        }
        RenderState::Loaded(value) => value,
        RenderState::Error(err) => return rsx! { "Error loading data: {err}" },
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
        {top_panel}
        table { class: "table table-striped",
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
        h1 { "Loading..." }
    }
}

fn load_log_items(env: Rc<String>, search_panel_state: SearchPanelState) {
    spawn(async move {
        let mut main_state = consume_context::<Signal<MainState>>();
        main_state.write().logs_data.set_loading();
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

fn load<'s>(
    env: Rc<String>,
    time_range: &TimeRange,
    mut main_state: Signal<MainState>,
    context_keys: Vec<LogEventContextApiModel>,
) {
    {
        main_state.write().logs_data.set_loading();
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
        let result =
            crate::api::logs::load_logs(env.to_string(), level, from, to, context_keys).await;

        match result {
            Ok(result) => {
                let items: Vec<Rc<LogApiItem>> = result.into_iter().map(Rc::new).collect();
                main_state.write().logs_data.set_value(items);
            }

            Err(err) => {
                main_state.write().logs_data.set_error(err.to_string());
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
        let result = crate::api::logs::search_logs(env.to_string(), from, to, phrase).await;

        match result {
            Ok(result) => {
                main_state
                    .write()
                    .logs_data
                    .set_value(result.into_iter().map(Rc::new).collect());
            }
            Err(err) => {
                main_state.write().logs_data.set_error(err.to_string());
            }
        }
    });
}
