pub use log_level::STORAGE_LEVEL_KEY;
pub use search_line::STORAGE_SEARCH_LINE_KEY;
pub use time_range::TIME_RANGE_KEY;

pub mod ctx_search;
pub mod log_level;
pub mod search_line;
pub mod time_range;

pub fn clean_all() {
    let web_local_storage = dioxus_utils::js::GlobalAppSettings::get_local_storage();
    web_local_storage.delete(STORAGE_SEARCH_LINE_KEY);
    web_local_storage.delete(STORAGE_LEVEL_KEY);
    web_local_storage.delete(TIME_RANGE_KEY);
}
