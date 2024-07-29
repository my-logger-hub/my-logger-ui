use std::rc::Rc;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::main_state::MainState;

#[component]
pub fn RenderDashboard() -> Element {
    let mut selected_state = use_signal(|| 60i64);

    let main_state = consume_context::<Signal<MainState>>();

    let dashboard_data = main_state.read().dashboard_data.clone();

    let env = main_state.read().selected_env.clone();
    let env_on_click = env.clone();

    let select = rsx! {
        select {
            class: "form-control",
            value: selected_state.read().to_string(),
            onchange: move |e| {
                let mut main_state = consume_context::<Signal<MainState>>();
                let value = e.value().parse::<i64>().unwrap();
                selected_state.set(value);
                main_state.write().set_dashboard_data(None);
                request_data(env_on_click.clone(), main_state, &selected_state);
            },
            option { value: "60", "1 Hour" }
            option { value: "120", "2 Hours" }
            option { value: "180", "3 Hours" }
            option { value: "240", "4 Hours" }
            option { value: "1440", "1 Day" }
        }
    };

    //let data_access = data.get();
    if dashboard_data.is_none() {
        request_data(env.clone(), main_state, &selected_state);

        return rsx! {
            {select},
            h1 { "Loading..." }
        };
    }

    let dashboard_data = dashboard_data.unwrap();

    let mut javascript = format!(
        "var yValues = [{}, {}, {}, {}, {}];",
        dashboard_data.info_count,
        dashboard_data.warning_count,
        dashboard_data.error_count,
        dashboard_data.fatal_count,
        dashboard_data.debug_count,
    );

    javascript.push_str(
        r#"
    
    var xValues = ["Info", "Warning", "Error", "Fatal Error", "Debug"];
    var barColors = ["green", "orange","red","black","yellow"];

    new Chart("myChart", {
        type: "pie",
        data: {
          labels: xValues,
          datasets: [{
            backgroundColor: barColors,
            data: yValues
          }]
        },
        options: {
          title: {
            display: true,
            text: "Logs Statistics"
          }
        }
      });"#,
    );

    rsx! {
        {select},
        div { style: "width:400px; height:400px",
            canvas { id: "myChart", style: "width:400px; height:400px" }
            script { {javascript} }
        }
    }
}

/*
fn request_data<'s>(cx: &'s Scope<'s>, data: &UseState<Option<StatisticData>>, hours: i64) {
    let data = data.to_owned();
    cx.spawn(async move {
        let grpc_client = APP_CTX.get_my_logger_grpc_client().await;

        let mut from_time = DateTimeAsMicroseconds::now();

        from_time.add_hours(-hours);

        let a = grpc_client
            .get_statistic(GetStatisticsRequest {
                tenant_id: "Default".to_string(),
                from_time: from_time.unix_microseconds,
                to_time: 0,
            })
            .await
            .unwrap();

        data.set(Some(a));
    });
}
 */

fn request_data(
    env: Rc<String>,
    mut main_state: Signal<MainState>,
    selected_state: &Signal<i64, UnsyncStorage>,
) {
    let minutes_before = *selected_state.read();

    spawn(async move {
        let result = get_dashboard(env.to_string(), minutes_before)
            .await
            .unwrap();

        main_state.write().set_dashboard_data(Some(result));
    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardItem {
    pub info_count: usize,
    pub warning_count: usize,
    pub error_count: usize,
    pub fatal_count: usize,
    pub debug_count: usize,
}

#[server]
pub async fn get_dashboard(
    env: String,
    minutes_before: i64,
) -> Result<DashboardItem, ServerFnError> {
    use crate::my_logger_grpc::*;
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    let mut from_time = DateTimeAsMicroseconds::now();

    from_time.add_minutes(-minutes_before);

    let result = crate::APP_CTX
        .get_client(env.as_str())
        .await
        .get_statistic(GetStatisticsRequest {
            tenant_id: "Default".to_string(),
            from_time: from_time.unix_microseconds,
            to_time: 0,
        })
        .await
        .unwrap();

    Ok(DashboardItem {
        info_count: result.info_count as usize,
        warning_count: result.warning_count as usize,
        error_count: result.error_count as usize,
        fatal_count: result.fatal_count as usize,
        debug_count: result.debug_count as usize,
    })
}
