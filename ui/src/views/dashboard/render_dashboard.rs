use std::rc::Rc;

use dioxus::prelude::*;

use crate::states::*;

use dioxus_utils::*;

#[component]
pub fn RenderDashboard() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let env = main_state_read_access.get_selected_env();
    let time_zone = main_state.read().get_selected_timezone();

    let dashboard_data = match main_state_read_access.dashboard_data.as_ref() {
        RenderState::None => {
            request_data(env, main_state);

            return rsx! {
                h1 { "Loading..." }
            };
        }
        RenderState::Loading => {
            return rsx! {
                h1 { "Loading..." }
            };
        }
        RenderState::Loaded(value) => value,
        RenderState::Error(err) => return rsx! { "Error loading data: {err}" },
    };

    let hourly_graph = super::render_hourly_graph(&dashboard_data.hourly, time_zone);

    let top_apps_with_errors =
        super::render_top_apps_with_errors(&dashboard_data.hourly, time_zone);

    rsx! {
        {hourly_graph}
        {top_apps_with_errors}
    }
}

fn request_data(env: Rc<String>, mut main_state: Signal<MainState>) {
    spawn(async move {
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
