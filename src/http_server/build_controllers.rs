use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

use super::controllers::*;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(Arc::new(app_info::GetAppInfoAction::new(app.clone())));

    result.register_get_action(Arc::new(server_info::GetServerInfoAction::new(app.clone())));

    result.register_get_action(Arc::new(dashboard::GetHourlyStatisticsAction::new(
        app.clone(),
    )));

    result.register_post_action(Arc::new(logs::LoadLogsAction::new(app.clone())));
    result.register_post_action(Arc::new(logs::SearchLogsAction::new(app.clone())));

    result.register_get_action(Arc::new(insights::GetInsightsKeysAction::new(app.clone())));

    result.register_get_action(Arc::new(ignore_events::GetIgnoreEventsAction::new(
        app.clone(),
    )));
    result.register_post_action(Arc::new(ignore_events::AddIgnoreEventAction::new(
        app.clone(),
    )));
    result.register_post_action(Arc::new(ignore_events::DeleteIgnoreEventAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(
        one_time_ignore_events::GetOneTimeIgnoreEventsAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        one_time_ignore_events::SaveOneTimeIgnoreEventAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        one_time_ignore_events::DeleteOneTimeIgnoreEventAction::new(app.clone()),
    ));

    result
}
