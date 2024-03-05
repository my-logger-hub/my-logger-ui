use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub fn render_dashboard() -> Element {
    let mut data: Signal<Option<DashboardItem>, _> = use_signal(|| None);

    let mut selected_state = use_signal(|| 60i64);

    let select = rsx! {
        select {
            class: "form-control",
            value: selected_state.read().to_string(),
            onchange: move |e| {
                let value = e.value().parse::<i64>().unwrap();
                selected_state.set(value);
                data.set(None);
                request_data(&mut data, &selected_state);
            },
            option { value: "60", "1 Hour" }
            option { value: "120", "2 Hours" }
            option { value: "180", "3 Hours" }
            option { value: "240", "4 Hours" }
            option { value: "1440", "1 Day" }
        }
    };

    //let data_access = data.get();
    if data.read().is_none() {
        request_data(&mut data, &selected_state);

        return rsx! {
            {select},
            h1 { "Loading..." }
        };
    }

    let data_access = data.read();
    let data_access = data_access.as_ref().unwrap();

    let mut javascript = format!(
        "var yValues = [{}, {}, {}, {}, {}];",
        data_access.info_count,
        data_access.warning_count,
        data_access.error_count,
        data_access.fatal_count,
        data_access.debug_count,
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
    data: &mut Signal<Option<DashboardItem>, UnsyncStorage>,
    selected_state: &Signal<i64, UnsyncStorage>,
) {
    let mut data = data.to_owned();

    let minutes_before = *selected_state.read();

    spawn(async move {
        let result = get_dashboard(minutes_before).await.unwrap();
        data.set(Some(result));
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
pub async fn get_dashboard(minutes_before: i64) -> Result<DashboardItem, ServerFnError> {
    use crate::my_logger_grpc::*;
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    let mut from_time = DateTimeAsMicroseconds::now();

    from_time.add_minutes(-minutes_before);

    let result = crate::APP_CTX
        .grpc_client
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
