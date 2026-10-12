use std::rc::Rc;

use dioxus::prelude::*;

use crate::MainState;

/// The dashboard, the logs and the settings all need the product scopes of the env: the one
/// opened first requests them
pub fn load_product_scopes(env: Rc<String>, mut main_state: Signal<MainState>) {
    spawn(async move {
        // A render which comes before this runs asks for them once more
        if !main_state.peek().product_scopes.is_none() {
            return;
        }

        main_state.write().product_scopes.set_loading();

        match crate::api::product_scopes::get(env.to_string()).await {
            Ok(result) => main_state.write().product_scopes.set_value(Rc::new(result)),
            Err(err) => main_state.write().product_scopes.set_error(err.to_string()),
        }
    });
}
