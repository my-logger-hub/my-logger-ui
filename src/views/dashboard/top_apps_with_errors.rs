use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::Route;

use super::HourlyStatisticsHttpModel;
pub fn render_top_apps_with_errors(hourly_statistics: &[HourlyStatisticsHttpModel]) -> Element {
    let mut by_hour_keys = BTreeMap::new();

    let mut max_errors = 0;

    for itm in hourly_statistics {
        if itm.error == 0 && itm.warning == 0 && itm.fatal == 0 {
            continue;
        }

        if !by_hour_keys.contains_key(&itm.hour_key) {
            by_hour_keys.insert(itm.hour_key, BTreeMap::default());
        }

        if itm.error > max_errors {
            max_errors = itm.error;
        }

        if itm.warning > max_errors {
            max_errors = itm.warning;
        }

        if itm.fatal > max_errors {
            max_errors = itm.fatal;
        }

        by_hour_keys.get_mut(&itm.hour_key).unwrap().insert(
            itm.app.to_string(),
            TotalStatistics {
                errors: itm.error,
                fatal_errors: itm.fatal,
                warnings: itm.warning,
            },
        );
    }

    let mut items_to_render = Vec::new();

    for (hour_key, items) in by_hour_keys.into_iter().rev() {
        items_to_render.push(rsx! {
            tr { style: "background-color: lightgray;font-weight: bold;    box-shadow: 0 3px 3px #00000012;",
                td { {super::hour_key_to_string(hour_key)} }
                td {}
                td {}
                td {}
            }
        });

        let mut sorted_items = BTreeMap::new();

        for (app, stat_data) in items {
            sorted_items.insert(stat_data.total(), (app, stat_data));
        }

        for (_, (app, stat_data)) in sorted_items.into_iter().rev() {
            let (err_width, fatal_err_width, warn_width) = if max_errors == 0 {
                (0.0, 0.0, 0.0)
            } else {
                let err_width = stat_data.errors as f64 / max_errors as f64 * 100.0;
                let fatal_err_width = stat_data.fatal_errors as f64 / max_errors as f64 * 100.0;
                let warn_width = stat_data.warnings as f64 / max_errors as f64 * 100.0;

                (err_width, fatal_err_width, warn_width)
            };

            items_to_render.push(rsx! {
                tr {
                    td { {app.as_str()} }
                    td {
                        div {
                            "Errors: "
                            Link {
                                to: Route::Logs {
                                    app: app.to_string(),
                                    level: "error".to_string(),
                                },
                                {stat_data.errors.to_string()}
                            }
                            div { class: "graph-wrapper",
                                div {
                                    class: "graph-errors",
                                    style: "width: {err_width}%"
                                }
                            }
                        }
                    }
                    td {
                        div {
                            "Fatal errors: "

                            Link {
                                to: Route::Logs {
                                    app: app.to_string(),
                                    level: "fatal".to_string(),
                                },
                                {stat_data.fatal_errors.to_string()}
                            }
                            div { class: "graph-wrapper",
                                div {
                                    class: "graph-fatal-errors",
                                    style: "width: {fatal_err_width}%"
                                }
                            }
                        }
                    }

                    td {
                        div {
                            "Warnings: "
                            Link {
                                to: Route::Logs {
                                    app: app.to_string(),
                                    level: "warning".to_string(),
                                },
                                {stat_data.warnings.to_string()}
                            }
                            div { class: "graph-wrapper",
                                div {
                                    class: "graph-fatal-warnings",
                                    style: "width: {warn_width}%"
                                }
                            }
                        }
                    }
                }
            });
        }
    }

    rsx! {
        table { class: "table table-striped table-bordered",
            tr {
                th {
                }
                th { "Errors" }
                th { "Fatal errors" }
                th { "Warnings" }
            }

            {items_to_render.into_iter()}
        }
    }
}

struct TotalStatistics {
    pub errors: u32,
    pub fatal_errors: u32,
    pub warnings: u32,
}

impl TotalStatistics {
    pub fn total(&self) -> u32 {
        self.errors + self.fatal_errors
    }
}
