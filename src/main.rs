#![allow(non_snake_case)]

mod states;

mod api;
mod js_bridge;
mod models;
#[cfg(feature = "server")]
mod server;

use crate::{dialogs::*, states::*, views::*};

use dioxus::prelude::*;

use dioxus_utils::*;
mod components;

mod insights;
mod log_event_context_parser;
mod storage_settings;

mod views;

mod dialogs;

const IGNORE_SINGLE_TIME_SUB_PATH: &str = "ignore-single-time";

// let cfg = dioxus::fullstack::Config::new().addr(([0, 0, 0, 0], 8080));

#[derive(Routable, PartialEq, Clone)]
enum AppRoute {
    #[route("/")]
    Home {},

    #[route("/dashboard/:env_name")]
    Dashboard { env_name: String },

    #[route("/logs/:..data")]
    Logs { data: Vec<String> },

    #[route("/ignoreLists/:..data")]
    IgnoreLists { data: Vec<String> },

    #[route("/settings")]
    Settings {},
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only!(ServeConfig::builder().incremental(
            dioxus_server::IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        )))
        .launch(|| {
            rsx! {
                document::Link { rel: "icon", href: asset!("/public/favicon.ico") }
                Router::<AppRoute> {}
            }
        })
}

#[component]
fn Home() -> Element {
    use_context_provider(|| Signal::new(LocationState::Dashboard));
    App()
}

#[component]
fn Logs(data: Vec<String>) -> Element {
    use models::*;
    use_context_provider(|| Signal::new(LocationState::Logs));

    if let Some(data) = data.get(0) {
        if let Some(model) = LogPathDataModel::from_base_64(data.as_str()) {
            //            let model_str = format!("model: {:?}", model);
            //            dioxus_utils::js::console_log(model_str.as_str());

            crate::storage_settings::log_level::set(model.get_log_level());
            crate::storage_settings::search_line::set(&model.search_string);
            crate::storage_settings::ctx_search::set(model.is_ctx_search);
            crate::storage_settings::time_range::set_as_str(&model.time_range);
        }
    }

    //crate::storage_settings::clean_all();
    App()
}

#[component]
fn Dashboard(env_name: String) -> Element {
    use_context_provider(|| Signal::new(LocationState::Dashboard));

    let mut envs_to_load_state = use_signal(|| DataState::default());

    let cs_ra = envs_to_load_state.read();

    let envs = match cs_ra.as_ref() {
        RenderState::None => {
            let origin = dioxus_utils::js::GlobalAppSettings::new()
                .get_origin()
                .to_string();
            spawn(async move {
                envs_to_load_state.write().set_loading();
                let result = crate::api::server_info::get_envs(origin).await;

                match result {
                    Ok(result) => {
                        envs_to_load_state.write().set_loaded(result);
                    }
                    Err(err) => {
                        envs_to_load_state.write().set_error(err.to_string());
                    }
                }
            });
            return rsx! { "[0]Loading envs..." };
        }
        RenderState::Loading => return rsx! { "[1]Loading envs..." },

        RenderState::Loaded(envs) => envs,

        RenderState::Error(err) => {
            let err = format!("Error loading envs: {}", err);
            return rsx! {
                {err}
            };
        }
    };

    if envs.iter().find(|x| *x == &env_name).is_none() {
        return rsx! { "Env not found" };
    }

    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .set(ENV_LOCAL_STORAGE_KEY, env_name.as_str());

    //crate::storage_settings::clean_all();
    App()
}

#[component]
fn IgnoreLists(data: Vec<String>) -> Element {
    if let Some(data) = data.get(0) {
        if data == IGNORE_SINGLE_TIME_SUB_PATH {
            use_context_provider(|| Signal::new(LocationState::OneTimeIgnore));
            return App();
        }
    }
    use_context_provider(|| Signal::new(LocationState::IgnoreList));
    App()
}

#[component]
fn Settings() -> Element {
    use_context_provider(|| Signal::new(LocationState::Settings));
    App()
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    use_context_provider(|| Signal::new(DialogState::None));
    let mut main_state = consume_context::<Signal<MainState>>();

    let has_envs = {
        let main_state = main_state.read();
        main_state.has_envs()
    };

    if has_envs {
        return rsx! {
            ActiveApp {}
        };
    }

    let resource = use_resource(|| {
        let origin = dioxus_utils::js::GlobalAppSettings::new()
            .get_origin()
            .to_string();

        crate::api::server_info::get_envs(origin)
    });

    let data = resource.read_unchecked();

    match &*data {
        Some(data) => match data {
            Ok(result) => {
                let time_zone = crate::js_bridge::get_time_zone();

                main_state
                    .write()
                    .set_environments(result.clone(), time_zone.into());
                return rsx! {
                    ActiveApp {}
                };
            }
            Err(err) => {
                let err = format!("Error loading environments. Err: {}", err);
                return rsx! {
                    {err}
                };
            }
        },

        None => {
            return rsx! { "Loading environments..." };
        }
    }
}

#[component]
fn ActiveApp() -> Element {
    let location_state_value = {
        let location_state = consume_context::<Signal<LocationState>>();
        let value = location_state.read();
        value.copy_state()
    };

    let right_panel = match location_state_value {
        LocationState::Dashboard => rsx! {
            RenderDashboard {}
        },
        LocationState::Logs => rsx! {
            RenderLogs {}
        },
        LocationState::IgnoreList => rsx! {
            RenderIgnoreListsRoot {}
        },
        LocationState::OneTimeIgnore => rsx! {
            RenderIgnoreListsRoot {}
        },

        LocationState::Settings => rsx! {
            RenderSettings {}
        },
    };

    rsx! {
        div { id: "left-panel",
            div { style: "margin: 5px;", EnvsSelector {} }
            LeftPanel {}
        }
        div { id: "main-panel", {right_panel} }
        RenderDialog {}
    }
}
