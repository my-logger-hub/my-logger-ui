use rest_api_shared::OneTimeIgnoreHttpModel;

pub trait OneTimeIgnoreHttpModelExt {
    /// An empty item with a fresh id - the server creates the item on save.
    fn create_new() -> Self;
}

impl OneTimeIgnoreHttpModelExt for OneTimeIgnoreHttpModel {
    fn create_new() -> Self {
        Self {
            id: rust_extensions::uuid::generate_v4(),
            levels: vec![],
            message_match: String::new(),
            ctx_match: None,
            skip_amount: 0,
            minutes_to_wait: 0,
        }
    }
}
