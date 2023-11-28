use std::rc::Rc;

use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    my_logger_grpc::{LogEventGrpcModel, LogLevelGrpcModel, ReadLogEventRequest},
    APP_CTX,
};

#[derive(Debug, Clone, Copy)]
pub enum SelectedLevel {
    All,
    Info,
    Warning,
    Error,
    FatalError,
    Debug,
}

#[derive(Clone, Copy)]
pub struct HoursAgo(i64);

impl HoursAgo {
    pub fn get_value(&self) -> i64 {
        self.0
    }
}

pub fn render_logs(cx: Scope) -> Element {
    let logs_state: &UseState<Option<Vec<Rc<LogEventGrpcModel>>>> = use_state(cx, || None);

    let log_level_filter: &UseState<SelectedLevel> = use_state(cx, || SelectedLevel::All);

    let hours_ago_filter: &UseState<HoursAgo> = use_state(cx, || HoursAgo(2));

    let log_state_value = logs_state.get();

    let log_level_value_as_str = format!("{:?}", log_level_filter.get());

    let panel = rsx! {

        table { style: "width: calc(var(--app-width) - var(--panel-width)); border-bottom: 1px lightgray solid; box-shadow: 0 0 5px lightgray; position: fixed; background:white",
            tr {
                td {
                    div { class: "input-group input-group-sm",
                        span { class: "input-group-text", "Level:" }

                        select {

                            class: "form-control",

                            onchange: move |e| {
                                match e.value.as_str() {
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
                            value: "{hours_ago_filter.get().get_value()}",
                            oninput: move |e| {
                                if let Ok(value) = e.value.parse::<i64>() {
                                    hours_ago_filter.set(HoursAgo(value));
                                }
                            }
                        }
                    }
                }
                td { style: "width:70%",
                    div { class: "input-group input-group-sm",
                        span { class: "input-group-text", "Key Value Filter:" }
                        input {
                            class: "form-control",
                            placeholder: "Example: Application='MyApp' AND Version='Version'"
                        }
                    }
                }
                td {
                    button {
                        class: "btn btn-primary btn-sm",
                        onclick: move |_| {
                            logs_state.set(None);
                        },
                        "Get data"
                    }
                }
            }
        }
    };

    if log_state_value.is_none() {
        load(
            &cx,
            log_level_filter.get().clone(),
            hours_ago_filter.get().get_value(),
            logs_state,
        );
        return render! {
            panel,
            h1 { "Loading" }
        };
    }

    let log_state_value = log_state_value.as_ref().unwrap();

    let items = log_state_value.iter().map(|itm| {
        let itm = itm.clone();

        let dt = DateTimeAsMicroseconds::new(itm.timestamp);

        let style = match itm.level() {
            crate::my_logger_grpc::LogLevelGrpcModel::Info => "green-ball",
            crate::my_logger_grpc::LogLevelGrpcModel::Warning => "orange-ball",
            crate::my_logger_grpc::LogLevelGrpcModel::Error => "red-ball",
            crate::my_logger_grpc::LogLevelGrpcModel::Fatal => "dark-red-ball",
            crate::my_logger_grpc::LogLevelGrpcModel::Debug => "yellow-ball",
        };

        let key_values: Vec<_> = itm
            .ctx
            .iter()
            .map(|ctx| {
                let key = ctx.key.to_string();
                let value = ctx.value.to_string();
                rsx! { div { style: "margin:0;padding:0", "{key}: '{value}'" } }
            })
            .collect();

        rsx! {

            tr { style: "border-top: 1px solid lightgray;",
                td { div { class: style } }
                td { style: "margin:0;padding:0", "{&dt.to_rfc3339()[..26]}" }
                td { style: "margin:0;padding:0", "{itm.process_name}" }
                td { style: "margin:0;padding:0", "{itm.message}" }
                td { key_values.into_iter() }
            }
        }
    });

    render! {
        panel,
        table { class: "table table-striped", style: "margin-top:36px",
            tr {
                th { style: "width: 24px;" }
                th { "Time" }
                th { "Process" }
                th { "Message" }
                th { "Context" }
            }
            items
        }
    }
}

fn load<'s>(
    cx: &'s Scope<'s>,
    log_level_filter: SelectedLevel,
    hours_before: i64,
    logs_state: &UseState<Option<Vec<Rc<LogEventGrpcModel>>>>,
) {
    let logs_state = logs_state.to_owned();

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
                context_keys: vec![],
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
