use my_http_utils::macros::MyHttpInput;

/// Input of every read endpoint which needs nothing but the environment to read from.
#[derive(MyHttpInput)]
pub struct EnvHttpRequest {
    #[http_query(
        name = "env",
        description = "Environment name as it is in the settings"
    )]
    pub env: String,
}
