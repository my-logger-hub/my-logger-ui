use std::rc::Rc;

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

use crate::states::*;

#[component]
pub fn RenderDashboard() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let env = main_state_read_access.get_selected_env();
    let time_zone = main_state.read().get_selected_timezone();

    let dashboard_data = match main_state_read_access.dashboard_data.clone() {
        DataState::None => {
            drop(main_state_read_access);
            main_state.write().dashboard_data = DataState::Loading;
            request_data(env, main_state);

            return rsx! {
                h1 { "Loading..." }
            };
        }
        DataState::Loading => {
            return rsx! {
                h1 { "Loading..." }
            };
        }
        DataState::Loaded(value) => value,
    };

    let hourly_graph = super::render_hourly_graph(&dashboard_data.hourly, time_zone);

    let top_apps_with_errors =
        super::render_top_apps_with_errors(&dashboard_data.hourly, time_zone);

    rsx! {
        {hourly_graph},
        {top_apps_with_errors}
    }
}

fn request_data(env: Rc<String>, mut main_state: Signal<MainState>) {
    spawn(async move {
        let result = get_dashboard(env.to_string()).await.unwrap();
        main_state.write().dashboard_data = DataState::Loaded(result);
    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardItem {
    pub hourly: Vec<HourlyStatisticsHttpModel>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HourlyStatisticsHttpModel {
    pub hour_key: i64,
    pub app: String,
    pub info: u32,
    pub warning: u32,
    pub error: u32,
    pub fatal: u32,
    pub debug: u32,
}

#[server]
pub async fn get_dashboard(env: String) -> Result<DashboardItem, ServerFnError> {
    use crate::my_logger_grpc::*;

    let client = crate::APP_CTX.get_client(env.as_str()).await;

    let items = client
        .get_hourly_statistics(GetHourlyStatisticsRequest { amount_of_hours: 8 })
        .await
        .unwrap()
        .unwrap_or_default();

    Ok(DashboardItem {
        hourly: items
            .into_iter()
            .map(|itm| HourlyStatisticsHttpModel {
                hour_key: itm.hour_key as i64,
                app: itm.app,
                info: itm.info_count,
                warning: itm.warning_count,
                error: itm.error_count,
                fatal: itm.fatal_count,
                debug: itm.debug_count,
            })
            .collect(),
    })
}
