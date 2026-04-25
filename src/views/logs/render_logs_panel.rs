use std::rc::Rc;

use dioxus::prelude::*;

use crate::{
    dialogs::DialogState, models::LogPathDataModel, storage_settings::log_level::SelectedLevel,
    views::logs::SelectLogLevel, *,
};

use crate::models::*;

#[component]
pub fn RenderLogsPanel(
    env: Rc<String>,
    time_zone: TimeZone,
    on_refresh_click: EventHandler<SearchPanelState>,
) -> Element {
    let mut insight_keys: Signal<DataState<Vec<String>>> = use_signal(|| DataState::default());

    if insight_keys.read().is_none() {
        spawn(async move {
            insight_keys.write().set_loading();
            let keys = crate::api::insights::get(env.to_string()).await.unwrap();
            insight_keys.write().set_value(keys);
        });
    }

    let mut search_panel_state = consume_context::<Signal<SearchPanelState>>();
    let search_panel_state_read_access = search_panel_state.read();

    let range_label = match search_panel_state_read_access.time_range.as_ref() {
        TimeRange::HoursAgo(_) => "From Hours ago:",
        TimeRange::Range(_, _) => "From Time range:",
        TimeRange::ExactHour(_) => "Get log from Hour:",
    };

    let search_placeholder = match search_panel_state_read_access.search_type {
        SearchType::Ctx => "Example: Application='MyApp' ; Version='Version'",
        SearchType::Text => "Just a text",
    };

    let level = search_panel_state_read_access.log_level.clone();

    let time_range_value = search_panel_state_read_access
        .time_range
        .to_string(time_zone);

    let is_ctx_search = search_panel_state_read_access.search_type.is_ctx_search();

    let second_path = LogPathDataModel {
        is_ctx_search,
        search_string: search_panel_state_read_access.filter.clone(),
        level: crate::storage_settings::log_level::get().into(),
        time_range: time_range_value.to_string(),
    }
    .to_base_64();

    rsx! {

        table { style: "width: calc(var(--app-width) - var(--panel-width)); border-bottom: 1px lightgray solid; box-shadow: 0 0 5px lightgray; position: fixed; background:white",
            tr {
                td { style: "width: 150px;",
                    div { style: "margin-top: 5px;", "Level" }
                    SelectLogLevel {
                        value: level.clone(),
                        on_change: move |level: SelectedLevel| {
                            search_panel_state.write().log_level = level.clone();
                            crate::storage_settings::log_level::set(level);
                        },
                    }
                }
                td { style: "width: 260px;",

                    div { style: "margin-top: 5px;", {range_label} }
                    input {
                        style: "width: 100%; cursor: pointer;",
                        class: "form-control form-control-sm",
                        readonly: true,
                        value: time_range_value.to_string(),
                        onclick: move |_| {
                            let value = { search_panel_state.read().time_range.clone() };
                            consume_context::<Signal<DialogState>>()
                                .set(DialogState::EditTimeRange {
                                    value,
                                    time_zone,
                                    on_change: EventHandler::new(move |time_range: TimeRange| {
                                        crate::storage_settings::time_range::set(&time_range);
                                        search_panel_state.write().time_range = time_range;
                                    }),
                                });
                        },
                    }
                }

                td { style: "width:60%;padding-left: 0px;",
                    div {
                        select {
                            style: "width: 100px; border: 1px solid white;",
                            class: "form-select form-select-sm",
                            oninput: move |e| {
                                let value = match e.value().as_str() {
                                    "ctx" => SearchType::Ctx,
                                    "text" => SearchType::Text,
                                    _ => SearchType::Ctx,
                                };
                                crate::storage_settings::ctx_search::set(value.is_ctx_search());
                                search_panel_state.write().search_type = value;
                            },
                            option { selected: is_ctx_search, value: "ctx", "Ctx Search" }
                            option { selected: !is_ctx_search, value: "text", "Text Search" }
                        }
                    }
                    input {
                        class: "form-control form-control-sm",
                        placeholder: search_placeholder,
                        value: "{search_panel_state_read_access.filter.as_str()}",
                        oninput: move |e| {
                            search_panel_state.write().filter = e.value();
                        },
                        onkeyup: move |e| {
                            if e.key() == Key::Enter {
                                let search_panel = {
                                    let mut main_state = consume_context::<Signal<MainState>>();
                                    main_state.write().logs_data.reset();
                                    search_panel_state.read().clone()
                                };
                                on_refresh_click.call(search_panel);
                            }
                        },
                    }

                    div { id: "insights-panel" }
                }
                td { style: "width: 32px;vertical-align: bottom;",
                    Link {
                        class: "btn btn-primary btn-sm",
                        to: AppRoute::Logs {
                            data: vec![second_path],
                        },
                        onclick: move |_| {
                            let search_panel = {
                                let mut main_state = consume_context::<Signal<MainState>>();
                                main_state.write().logs_data.reset();
                                search_panel_state.read().clone()
                            };
                            on_refresh_click.call(search_panel);
                        },
                        img {
                            src: "/assets/img/ico-refresh.svg",
                            style: "width: 16px;",
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPanelState {
    pub log_level: SelectedLevel,
    pub filter: String,
    pub time_range: TimeRange,
    pub search_type: SearchType,
    pub initialized: bool,
}

impl SearchPanelState {
    pub fn new() -> Self {
        Self {
            search_type: SearchType::Ctx,
            log_level: SelectedLevel::All,
            filter: String::new(),
            time_range: TimeRange::default(),
            initialized: false,
        }
    }

    pub fn read_storage() -> (SearchType, SelectedLevel, String, TimeRange) {
        let search_type = if crate::storage_settings::ctx_search::get() {
            SearchType::Ctx
        } else {
            SearchType::Text
        };
        let log_level = crate::storage_settings::log_level::get();
        let filter = crate::storage_settings::search_line::get();
        let time_range = crate::storage_settings::time_range::get();
        (search_type, log_level, filter, time_range)
    }

    pub fn matches_storage(&self) -> bool {
        if !self.initialized {
            return false;
        }
        let (search_type, log_level, filter, time_range) = Self::read_storage();
        self.search_type == search_type
            && self.log_level == log_level
            && self.filter == filter
            && self.time_range == time_range
    }

    pub fn load_from_storage(&mut self) {
        let (search_type, log_level, filter, time_range) = Self::read_storage();
        self.search_type = search_type;
        self.log_level = log_level;
        self.filter = filter;
        self.time_range = time_range;
        self.initialized = true;
    }

    pub fn append_filter_conditions(&mut self, key: &str, value: &str) {
        if !self.filter.is_empty() {
            self.filter.push_str(" and ");
        }
        self.filter.push_str(key);
        self.filter.push_str(":'");
        self.filter.push_str(value);
        self.filter.push_str("'");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Ctx,
    Text,
}

impl SearchType {
    pub fn is_ctx_search(&self) -> bool {
        match self {
            SearchType::Ctx => true,
            SearchType::Text => false,
        }
    }
}
