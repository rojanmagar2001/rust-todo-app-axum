use axum::{http::HeaderValue, Router};
use rust_todo_app_axum::{
    common::{error::ApiError, middleware::ApiContext},
    config::Config,
    user::routes as user_routes,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{add_extension::AddExtensionLayer, cors::CorsLayer, trace::TraceLayer};

pub fn build_routes(pool: Pool<Postgres>, config: Config) -> Router {
    let ctx = ApiContext {
        pool,
        config: Arc::new(config),
    };

    let api_routes = Router::new().nest("/api", user_routes::route());

    Router::new()
        .merge(api_routes)
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(ctx))
                .layer(TraceLayer::new_for_http()),
        )
        .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        .fallback(handler_404)
}

//.layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
//      // Log the matched route's path (with placeholders not filled in).
// Use request.uri() or OriginalUri if you wan the real path.
//    let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

//  info_span!("http_request", method = ?request.method(), matched_path, some_other_field = tracing::field::Empty)
// }))
//

async fn handler_404() -> ApiError {
    ApiError::new_not_found("Resource not found".into())
}
