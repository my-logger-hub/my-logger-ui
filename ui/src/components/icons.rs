use dioxus::prelude::*;

/// Inline SVG icon set ported from the design prototype (`components.jsx` → ICONS).
/// viewBox 0 0 16 16, stroke currentColor, stroke-width 1.5.
pub fn icon(name: &str, size: u32) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 16 16",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            {icon_body(name)}
        }
    }
}

fn icon_body(name: &str) -> Element {
    match name {
        "dashboard" => rsx! {
            rect { x: "2", y: "2", width: "5", height: "6", rx: "1" }
            rect { x: "9", y: "2", width: "5", height: "3", rx: "1" }
            rect { x: "9", y: "7", width: "5", height: "7", rx: "1" }
            rect { x: "2", y: "10", width: "5", height: "4", rx: "1" }
        },
        "logs" => rsx! {
            path { d: "M3 2.5h7l3 3V13a.5.5 0 0 1-.5.5h-9A.5.5 0 0 1 3 13V3a.5.5 0 0 1 .5-.5z" }
            path { d: "M9.5 2.5V6h3.5" }
            path { d: "M5 8h6M5 10.5h6M5 5.5h2.5" }
        },
        "ignore" => rsx! {
            circle { cx: "8", cy: "8", r: "5.5" }
            path { d: "M4.5 4.5l7 7" }
        },
        "settings" => rsx! {
            circle { cx: "8", cy: "8", r: "2" }
            path { d: "M8 1.5v2M8 12.5v2M3.4 3.4l1.4 1.4M11.2 11.2l1.4 1.4M1.5 8h2M12.5 8h2M3.4 12.6l1.4-1.4M11.2 4.8l1.4-1.4" }
        },
        "search" => rsx! {
            circle { cx: "7", cy: "7", r: "4.5" }
            path { d: "M10.5 10.5l3 3" }
        },
        "refresh" => rsx! {
            path { d: "M13 8a5 5 0 1 1-1.7-3.8" }
            path { d: "M13.5 2v3.5H10" }
        },
        "filter" => rsx! {
            path { d: "M2 3h12l-4.5 5.5V13l-3 1V8.5L2 3z" }
        },
        "plus" => rsx! {
            path { d: "M8 3v10M3 8h10" }
        },
        "minus" => rsx! {
            path { d: "M3 8h10" }
        },
        "x" => rsx! {
            path { d: "M3.5 3.5l9 9M12.5 3.5l-9 9" }
        },
        "clock" => rsx! {
            circle { cx: "8", cy: "8", r: "6" }
            path { d: "M8 4.5V8l2.5 1.5" }
        },
        "trash" => rsx! {
            path { d: "M3 4h10M6 4V2.5h4V4M5 4l.7 9.5a1 1 0 0 0 1 .9h2.6a1 1 0 0 0 1-.9L11 4" }
        },
        "pencil" => rsx! {
            path { d: "M11 2.5l2.5 2.5L5 13.5H2.5V11L11 2.5z" }
        },
        "warn" => rsx! {
            path { d: "M8 2L1.5 13.5h13L8 2z" }
            path { d: "M8 6.5v3.5" }
            circle { cx: "8", cy: "12", r: "0.5", fill: "currentColor" }
        },
        "share" => rsx! {
            circle { cx: "4", cy: "8", r: "1.5" }
            circle { cx: "12", cy: "4", r: "1.5" }
            circle { cx: "12", cy: "12", r: "1.5" }
            path { d: "M5.3 7.3l5.4-2.6M5.3 8.7l5.4 2.6" }
        },
        "sun" => rsx! {
            circle { cx: "8", cy: "8", r: "3" }
            path { d: "M8 1.5v1.5M8 13v1.5M2.5 8H4M12 8h1.5M3.8 3.8l1 1M11.2 11.2l1 1M3.8 12.2l1-1M11.2 4.8l1-1" }
        },
        "moon" => rsx! {
            path { d: "M13.5 9.5A6 6 0 1 1 6.5 2.5a5 5 0 0 0 7 7z" }
        },
        "inbox" => rsx! {
            path { d: "M2 9l2-5h8l2 5" }
            path { d: "M2 9v4.5h12V9H10a2 2 0 0 1-4 0H2z" }
        },
        "caret" => rsx! {
            path { d: "M4 6l4 4 4-4" }
        },
        _ => rsx! {
            circle { cx: "8", cy: "8", r: "1", fill: "currentColor" }
        },
    }
}
