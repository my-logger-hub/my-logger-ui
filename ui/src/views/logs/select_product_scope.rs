use std::rc::Rc;

use dioxus::prelude::*;

use crate::models::*;

/// `value` - the scope the logs are read from, empty for the default one of the env
#[component]
pub fn SelectProductScope(
    scopes: Rc<ProductScopesHttpModel>,
    value: String,
    on_change: EventHandler<String>,
) -> Element {
    let selected = if value.is_empty() {
        scopes.default_scope.as_str()
    } else {
        value.as_str()
    };

    let mut names: Vec<&str> = scopes.scopes.iter().map(|itm| itm.name.as_str()).collect();

    // A scope given by a link is the one the logs are read from even if the env does not list it
    if !names.contains(&selected) {
        names.insert(0, selected);
    }

    let items = names.into_iter().map(|name| {
        let label = if name == scopes.default_scope {
            format!("{} (default)", name)
        } else {
            name.to_string()
        };

        rsx! {
            option { selected: name == selected, value: name, {label} }
        }
    });

    let default_scope = scopes.default_scope.clone();

    rsx! {
        select {
            class: "form-select form-select-sm",
            oninput: move |e| {
                let value = e.value();
                // The default scope is kept as none - the env decides which one it is
                let value = if value == default_scope { String::new() } else { value };
                on_change.call(value);
            },
            {items}
        }
    }
}
