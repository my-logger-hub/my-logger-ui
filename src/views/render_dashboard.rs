use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    my_logger_grpc::{ReadLogEventRequest, StatisticData},
    APP_CTX,
};

pub fn render_dashboard(cx: Scope) -> Element {
    let data: &UseState<Option<StatisticData>> = use_state(cx, || None);

    let selected_state = use_state(cx, || 69i64);

    let data_access = data.get();

    let select = rsx! {
        select {
            class: "form-control",
            onchange: move |e| {
                let value = e.value.parse::<i64>().unwrap();
                selected_state.set(value);
                request_data(&cx, data, *selected_state.get());
            },
            option { value: "60", "1 Hour" }
            option { value: "120", "2 Hours" }
            option { value: "180", "3 Hours" }
            option { value: "240", "4 Hours" }
            option { value: "1440", "1 Day" }
        }
    };

    if data_access.is_none() {
        request_data(&cx, data, *selected_state.get());
        return render! {
            select,
            h1 { "Loading..." }
        };
    }

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

    render! {
        select,
        div { style: "width:400px; height:400px",
            canvas { id: "myChart", style: "width:400px; height:400px" }
            script { javascript }
        }
    }
}

fn request_data<'s>(cx: &'s Scope<'s>, data: &UseState<Option<StatisticData>>, hours: i64) {
    let data = data.to_owned();
    cx.spawn(async move {
        let grpc_client = APP_CTX.get_my_logger_grpc_client().await;

        let mut from_time = DateTimeAsMicroseconds::now();

        from_time.add_hours(-hours);

        let a = grpc_client
            .get_statistic(ReadLogEventRequest {
                tenant_id: "Default".to_string(),
                from_time: from_time.unix_microseconds,
                to_time: 0,
                levels: vec![],
                context_keys: vec![],
                skip: 0,
                take: 0,
            })
            .await
            .unwrap();

        data.set(Some(a));
    });
}
