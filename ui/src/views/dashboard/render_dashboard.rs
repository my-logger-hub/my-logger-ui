use std::rc::Rc;

use dioxus::prelude::*;

use crate::{states::*, views::load_product_scopes};

use dioxus_utils::*;

use super::HourlyGraph;

#[component]
pub fn RenderDashboard() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let env = main_state_read_access.get_selected_env();
    let time_zone = main_state_read_access.get_selected_timezone();

    // The statistics are shown by product scope, so both are requested - at once
    let product_scopes = main_state_read_access.product_scopes.as_ref();

    if let RenderState::None = product_scopes {
        load_product_scopes(env.clone(), main_state);
    }

    let dashboard_data = main_state_read_access.dashboard_data.as_ref();

    if let RenderState::None = dashboard_data {
        request_data(env, main_state);
    }

    let (product_scopes, dashboard_data) = match (product_scopes, dashboard_data) {
        (RenderState::Loaded(product_scopes), RenderState::Loaded(dashboard_data)) => {
            (product_scopes, dashboard_data)
        }
        (RenderState::Error(err), _) | (_, RenderState::Error(err)) => {
            return rsx! { "Error loading data: {err}" }
        }
        _ => {
            return rsx! {
                h1 { "Loading..." }
            };
        }
    };

    let default_scope = product_scopes.default_scope.as_str();

    let selected_scope = if main_state_read_access.product_scope.is_empty() {
        default_scope
    } else {
        main_state_read_access.product_scope.as_str()
    };

    let mut scopes: Vec<&str> = product_scopes
        .scopes
        .iter()
        .map(|itm| itm.name.as_str())
        .collect();

    for itm in &dashboard_data.hourly {
        if !scopes.contains(&itm.scope.as_str()) {
            scopes.push(itm.scope.as_str());
        }
    }

    // A scope given by a link is the one shown even if the env does not list it
    if !scopes.contains(&selected_scope) {
        scopes.push(selected_scope);
    }

    // Only when there is a scope to choose
    let tabs = (scopes.len() > 1).then(|| {
        let tabs = scopes.iter().map(|name| {
            let name = *name;

            let active = if name == selected_scope { "active" } else { "" };

            let errors: u32 = dashboard_data
                .hourly
                .iter()
                .filter(|itm| itm.scope == name)
                .map(|itm| itm.error + itm.fatal)
                .sum();

            // The default scope is kept as none - the env decides which one it is
            let value = if name == default_scope {
                String::new()
            } else {
                name.to_string()
            };

            rsx! {
                li { key: "{name}", class: "nav-item",
                    button {
                        class: "nav-link {active}",
                        r#type: "button",
                        onclick: move |_| main_state.write().set_product_scope(&value),
                        "{name}"
                        if name == default_scope {
                            " (default)"
                        }
                        if errors > 0 {
                            span {
                                class: "badge text-bg-danger ms-1",
                                title: "Errors and fatal errors of the hours shown",
                                "{errors}"
                            }
                        }
                    }
                }
            }
        });

        rsx! {
            ul { class: "nav nav-tabs", {tabs} }
        }
    });

    let rows = super::get_hourly_graph_rows(&dashboard_data.hourly, selected_scope, time_zone);

    let top_apps_with_errors =
        super::render_top_apps_with_errors(&dashboard_data.hourly, selected_scope, time_zone);

    rsx! {
        {tabs}
        HourlyGraph { rows }
        {top_apps_with_errors}
    }
}

fn request_data(env: Rc<String>, mut main_state: Signal<MainState>) {
    spawn(async move {
        // A render which comes before this runs asks for them once more
        if !main_state.peek().dashboard_data.is_none() {
            return;
        }

        main_state.write().dashboard_data.set_loading();
        let result = crate::api::dashboard::get_dashboard(env.to_string()).await;

        match result {
            Ok(result) => {
                main_state.write().dashboard_data.set_loaded(result);
            }
            Err(err) => {
                main_state.write().dashboard_data.set_error(err.to_string());
            }
        }
    });
}
