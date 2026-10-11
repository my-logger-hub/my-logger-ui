use rest_api_shared::OneTimeIgnoreHttpModel;

use crate::my_logger_grpc::{IgnoreSingleEventGrpcModel, LogEventContext, LogLevelGrpcModel};

impl From<IgnoreSingleEventGrpcModel> for OneTimeIgnoreHttpModel {
    fn from(value: IgnoreSingleEventGrpcModel) -> Self {
        let levels = value.levels().map(|itm| itm.into()).collect();

        // An empty context match goes over the wire as an absent one
        let ctx_match = if value.context_match.is_empty() {
            None
        } else {
            Some(
                value
                    .context_match
                    .into_iter()
                    .map(|itm| (itm.key, itm.value))
                    .collect(),
            )
        };

        Self {
            id: value.id,
            levels,
            message_match: value.message_match,
            ctx_match,
            skip_amount: value.skip_amount,
            minutes_to_wait: value.minutes_to_wait,
        }
    }
}

impl From<OneTimeIgnoreHttpModel> for IgnoreSingleEventGrpcModel {
    fn from(value: OneTimeIgnoreHttpModel) -> Self {
        Self {
            id: value.id,
            levels: value
                .levels
                .into_iter()
                .map(|itm| LogLevelGrpcModel::from(itm) as i32)
                .collect(),
            message_match: value.message_match,
            context_match: value
                .ctx_match
                .unwrap_or_default()
                .into_iter()
                .map(|(key, value)| LogEventContext { key, value })
                .collect(),
            skip_amount: value.skip_amount,
            minutes_to_wait: value.minutes_to_wait,
        }
    }
}
