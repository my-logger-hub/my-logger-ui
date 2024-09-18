use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum DataState<T: Debug + Clone> {
    None,
    Loading,
    Loaded(T),
}
