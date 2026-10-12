use dioxus::prelude::*;
use std::collections::BTreeMap;

use crate::models::*;

const DRAW_CHART: &str = r#"

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

/// The rows of the chart of a product scope. An hour another scope has statistics of is a row
/// as well, so every scope is drawn over the same hours
pub fn get_hourly_graph_rows(
    hourly_statistics: &[HourlyStatisticsHttpModel],
    product_scope: &str,
    time_zone: TimeZone,
) -> String {
    let mut hourly_data_to_render = BTreeMap::new();

    for itm in hourly_statistics.iter() {
        if !hourly_data_to_render.contains_key(&itm.hour_key) {
            hourly_data_to_render.insert(itm.hour_key, HourlyTotalData::default());
        }

        if itm.scope != product_scope {
            continue;
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
        let hour_key: DateHourKey = (*key).into();
        let hour_key_as_string = hour_key.to_local_time(time_zone).to_string();

        if js_error_line.len() > 0 {
            js_error_line.push_str(",");
        }
        js_error_line.push_str(
            format!(
                "['{}', {}, {}, {}, {}, {}]",
                hour_key_as_string,
                value.fatal,
                value.error,
                value.warning,
                value.info,
                value.debug
            )
            .as_str(),
        );
    }

    js_error_line
}

#[component]
pub fn HourlyGraph(rows: String) -> Element {
    // A script element runs once, when it gets into the page - and the chart is to be drawn
    // again with every other product scope chosen
    use_effect(use_reactive!(|rows| {
        dioxus_utils::eval(DRAW_CHART.replace("####", rows.as_str()).as_str());
    }));

    rsx! {
        div { id: "errorsChart", style: "width:100%; height:400px" }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn statistics(hour_key: i64, scope: &str, error: u32) -> HourlyStatisticsHttpModel {
        HourlyStatisticsHttpModel {
            hour_key,
            app: "app".to_string(),
            scope: scope.to_string(),
            info: 0,
            warning: 0,
            error,
            fatal: 0,
            debug: 0,
        }
    }

    #[test]
    fn rows_are_of_the_scope_over_the_hours_of_every_scope() {
        let hourly_statistics = vec![
            statistics(2026101208, "risks", 3),
            statistics(2026101209, "risks", 1),
            statistics(2026101209, "trading", 5),
            statistics(2026101209, "trading", 2),
        ];

        let rows =
            get_hourly_graph_rows(&hourly_statistics, "trading", TimeZone::create_utc_zero());

        assert_eq!(
            rows,
            "['2026-10-12T08:00:00', 0, 0, 0, 0, 0],['2026-10-12T09:00:00', 0, 7, 0, 0, 0]"
        );
    }
}
