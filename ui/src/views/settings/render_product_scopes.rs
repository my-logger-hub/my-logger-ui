use dioxus::prelude::*;

use dioxus_utils::*;

use crate::{views::load_product_scopes, MainState};

/// The product scopes of the env and the rules a record gets its scope by. They are set
/// through MCP of MyLogger server, so here they are only shown
#[component]
pub fn RenderProductScopes() -> Element {
    let main_state = consume_context::<Signal<MainState>>();
    let main_state_read_access = main_state.read();

    let product_scopes = match main_state_read_access.product_scopes.as_ref() {
        RenderState::None => {
            load_product_scopes(main_state_read_access.get_selected_env(), main_state);

            return rsx! {
                h4 { "Product scopes" }
                div { "Loading..." }
            };
        }
        RenderState::Loading => {
            return rsx! {
                h4 { "Product scopes" }
                div { "Loading..." }
            };
        }
        RenderState::Loaded(value) => value,
        RenderState::Error(err) => {
            return rsx! {
                h4 { "Product scopes" }
                div { "Error loading product scopes: {err}" }
            };
        }
    };

    let table_content = product_scopes.scopes.iter().map(|scope| {
        let is_default = scope.name == product_scopes.default_scope;

        let applications = scope.applications.iter().map(|application| {
            rsx! {
                div {
                    code { "{application}" }
                }
            }
        });

        let note = match (is_default, scope.applications.is_empty()) {
            (true, true) => "Everything no scope matches",
            (true, false) => "And everything no scope matches",
            // There are records of it, though no rule leads to it
            (false, true) => "No rules: only the records written with the scope header",
            (false, false) => "",
        };

        rsx! {
            tr {
                td {
                    "{scope.name}"
                    if is_default {
                        span { class: "badge text-bg-secondary ms-1", "default" }
                    }
                }
                td {
                    {applications}
                    if !note.is_empty() {
                        div { class: "text-muted", {note} }
                    }
                }
            }
        }
    });

    rsx! {
        h4 { "Product scopes" }
        div { class: "text-muted", style: "margin-bottom: 10px;",
            "A record goes to the scope its writer names in the scope header. Without it, the scopes are tried in the order they are set: the record goes to the first one with a regular expression matching its application - whole, ignoring case. Everything else goes to the default scope."
        }
        table { class: "table table-striped",
            thead {
                tr {
                    th { style: "width: 30%;", "Scope" }
                    th { "Applications" }
                }
            }
            tbody { {table_content} }
        }
    }
}
