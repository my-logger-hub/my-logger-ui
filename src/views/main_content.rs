use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{my_logger_grpc::ReadLogEventRequest, APP_CTX};

pub fn main_content(cx: Scope) -> Element {
    cx.spawn(async move {
        let grpc_client = APP_CTX.get_my_logger_grpc_client().await;

        let mut from_time = DateTimeAsMicroseconds::now();

        from_time.add_days(-10);

        let a = grpc_client
            .get_statistic(ReadLogEventRequest {
                tenant_id: "trx".to_string(),
                from_time: from_time.unix_microseconds,
                to_time: 0,
                levels: vec![],
                context_keys: vec![],
            })
            .await
            .unwrap();

        println!("{:?}", a);
    });

    render! { h1 { "Hello World!" } }
}
