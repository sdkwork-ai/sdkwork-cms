//! CMS app-api route manifest and adapter skeleton.

pub mod dto;
pub mod error;
pub mod handlers;
pub mod http_route_manifest;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use manifest::cms_app_api_manifest;
pub use routes::build_sdkwork_cms_app_api_router;

pub fn gateway_mount(state: sdkwork_cms_http_handlers::AppState) -> axum::Router {
    use axum::routing::get;
    use sdkwork_cms_http_handlers::handlers;

    axum::Router::new()
        .route(paths::BOOTSTRAP, get(handlers::delivery_bootstrap))
        .route(
            paths::ENTRIES_RESOLVE,
            get(handlers::delivery_resolve_entry),
        )
        .route(paths::ENTRY_BY_ID, get(handlers::delivery_retrieve_entry))
        .route(paths::PAGES_RESOLVE, get(handlers::delivery_resolve_page))
        .route(paths::FEED_ITEMS, get(handlers::delivery_list_feed_items))
        .with_state(state)
}
