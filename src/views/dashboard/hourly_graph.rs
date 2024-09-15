use dioxus::prelude::*;
use std::collections::BTreeMap;

use super::HourlyStatisticsHttpModel;

pub fn render_hourly_graph(hourly_statistics: &[HourlyStatisticsHttpModel]) -> Element {
    let mut hourly_data_to_render = BTreeMap::new();

    for itm in hourly_statistics.iter() {
        if !hourly_data_to_render.contains_key(&itm.hour_key) {
            hourly_data_to_render.insert(itm.hour_key, HourlyTotalData::default());
        }

        if let Some(data) = hourly_data_to_render.get_mut(&itm.hour_key) {
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
                "['{}', {}, {}, {}, {}, {}]",
                super::hour_key_to_string(*key),
                value.fatal,
                value.error,
                value.warning,
                value.info,
                value.debug
            )
            .as_str(),
        );
    }

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

#[derive(Default)]
pub struct HourlyTotalData {
    pub info: u32,
    pub warning: u32,
    pub error: u32,
    pub fatal: u32,
    pub debug: u32,
}
