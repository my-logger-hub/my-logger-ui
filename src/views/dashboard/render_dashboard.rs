use std::rc::Rc;

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

use crate::states::*;

#[component]
pub fn RenderDashboard() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let (dashboard_data, time_zone) = {
        let main_state_access = main_state.read();

        (
            main_state_access.dashboard_data.clone(),
            main_state_access.time_zone,
        )
    };

    let env = main_state.read().get_selected_env();

    //let data_access = data.get();
    if dashboard_data.is_none() {
        request_data(env.clone(), main_state, time_zone);

        return rsx! {
            h1 { "Loading..." }
        };
    }

    let dashboard_data = dashboard_data.unwrap();

    let hourly_graph = super::render_hourly_graph(&dashboard_data.hourly);

    let top_apps_with_errors = super::render_top_apps_with_errors(&dashboard_data.hourly);

    rsx! {
        {hourly_graph},
        {top_apps_with_errors}
    }
}

fn request_data(env: Rc<String>, mut main_state: Signal<MainState>, time_zone: i64) {
    spawn(async move {
        let result = get_dashboard(env.to_string(), time_zone).await.unwrap();

        main_state.write().set_dashboard_data(Some(result));
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
pub async fn get_dashboard(env: String, time_zone: i64) -> Result<DashboardItem, ServerFnError> {
    use crate::date_key::DateHourKey;
    use crate::my_logger_grpc::*;
    use rust_extensions::date_time::DateTimeAsMicroseconds;
    let client = crate::APP_CTX.get_client(env.as_str()).await;

    let items = client
        .get_hourly_statistics(GetHourlyStatisticsRequest { amount_of_hours: 8 })
        .await
        .unwrap()
        .unwrap_or_default();

    Ok(DashboardItem {
        hourly: items
            .into_iter()
            .map(|itm| {
                let key: DateHourKey = itm.hour_key.into();
                let mut dt: DateTimeAsMicroseconds = key.into();

                dt.add_minutes(-time_zone);

                let hour_key_local: DateHourKey = dt.into();

                HourlyStatisticsHttpModel {
                    hour_key: hour_key_local.get_value(),
                    app: itm.app,
                    info: itm.info_count,
                    warning: itm.warning_count,
                    error: itm.error_count,
                    fatal: itm.fatal_count,
                    debug: itm.debug_count,
                }
            })
            .collect(),
    })
}
