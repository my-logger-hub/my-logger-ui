use std::{collections::BTreeMap, rc::Rc};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::states::*;

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

    let mut hourly_data_to_render = BTreeMap::new();

    for itm in dashboard_data.hourly.iter() {
        if !hourly_data_to_render.contains_key(&itm.HourKey) {
            hourly_data_to_render.insert(itm.HourKey, HourlyTotalData::default());
        }

        if let Some(data) = hourly_data_to_render.get_mut(&itm.HourKey) {
            data.info += itm.info;
            data.warning += itm.warning;
            data.error += itm.error;
            data.fatal += itm.fatal;
            data.debug += itm.debug;
        }
    }

    let mut js_error_line = String::new();

    for (key, value) in hourly_data_to_render.iter() {
        if js_error_line.len() > 0 {
            js_error_line.push_str(",");
        }
        js_error_line.push_str(
            format!(
                "['{key}', {}, {}, {}, {}, {}]",
                value.fatal, value.error, value.warning, value.info, value.debug
            )
            .as_str(),
        );
    }

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

    let javascript_2 = r#"

      google.charts.load('current', {'packages':['bar']});

      google.charts.setOnLoadCallback(drawChart);

      function drawChart() {

         var data = google.visualization.arrayToDataTable([
          ['Amount','FatalError', 'Error',  'Warning', 'Info', 'Debug'],
          ####
        ]);

        var options = {
          chart: {
            title: 'Errors statistics',
            subtitle: 'per hour',
   
          },
          colors: ['black', 'red',  'orange', 'darkgreen', 'gray']
        };

        var chart = new google.charts.Bar(document.getElementById('errorsChart'));

        chart.draw(data, google.charts.Bar.convertOptions(options));
      }
    
    "#;

    let javascript_2 = javascript_2.replace("####", js_error_line.as_str());

    rsx! {
        div { id: "errorsChart", style: "width:100%; height:400px" }
        script { {javascript_2} }
    }
}

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

    pub hourly: Vec<HourlyStatisticsHttpModel>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HourlyStatisticsHttpModel {
    pub HourKey: u64,
    app: String,
    info: u32,
    warning: u32,
    error: u32,
    fatal: u32,
    debug: u32,
}

#[derive(Default)]
pub struct HourlyTotalData {
    info: u32,
    warning: u32,
    error: u32,
    fatal: u32,
    debug: u32,
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

    let client = crate::APP_CTX.get_client(env.as_str()).await;

    let result = client
        .get_statistic(GetStatisticsRequest {
            tenant_id: "Default".to_string(),
            from_time: from_time.unix_microseconds,
            to_time: 0,
        })
        .await
        .unwrap();

    let items = client
        .get_hourly_statistics(GetHourlyStatisticsRequest { amount_of_hours: 8 })
        .await
        .unwrap()
        .unwrap_or_default();

    Ok(DashboardItem {
        info_count: result.info_count as usize,
        warning_count: result.warning_count as usize,
        error_count: result.error_count as usize,
        fatal_count: result.fatal_count as usize,
        debug_count: result.debug_count as usize,

        hourly: items
            .into_iter()
            .map(|itm| HourlyStatisticsHttpModel {
                HourKey: itm.hour_key,
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
