pub const STORAGE_PRODUCT_SCOPE_KEY: &str = "product-scope";

/// The product scope to read the logs of. Empty - the default one of the env
pub fn get() -> String {
    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(STORAGE_PRODUCT_SCOPE_KEY)
        .unwrap_or_default()
}

pub fn set(product_scope: &str) {
    if product_scope.is_empty() {
        clear();
    } else {
        dioxus_utils::js::GlobalAppSettings::get_local_storage()
            .set(STORAGE_PRODUCT_SCOPE_KEY, product_scope);
    }
}

pub fn clear() {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().delete(STORAGE_PRODUCT_SCOPE_KEY);
}
