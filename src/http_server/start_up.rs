use std::{net::SocketAddr, sync::Arc};

use my_http_server::{
    controllers::swagger::SwaggerMiddleware, MyHttpServer, StaticFilesMiddleware,
};

use crate::app::{AppContext, APP_NAME, APP_VERSION};

/// The standard port of our services - the API and the UI both answer on it.
const HTTP_PORT: u16 = 8000;

pub fn start(app: &Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], HTTP_PORT)));

    let controllers = Arc::new(super::build_controllers(app));

    let swagger_middleware = SwaggerMiddleware::new(controllers.clone(), APP_NAME, APP_VERSION);

    // The UI from ./wwwroot (built by ./build-ui.sh). A route of the SPA is not a file, so it
    // gets index.html. ETag makes the browser revalidate the files after every deploy.
    let static_files_middleware = StaticFilesMiddleware::new()
        .add_index_file("index.html")
        .set_not_found_file("index.html".to_string())
        .with_etag()
        .enable_files_caching();

    http_server.add_middleware(Arc::new(swagger_middleware));
    http_server.add_middleware(controllers);
    http_server.add_middleware(Arc::new(static_files_middleware));

    http_server.start(app.states.clone(), my_logger::LOGGER.clone());
}
