//! CMS backend-api route manifest and adapter skeleton.

pub mod dto;
pub mod error;
pub mod handlers;
pub mod http_route_manifest;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use manifest::cms_backend_api_manifest;
pub use routes::build_sdkwork_cms_backend_api_router;

const OUTBOX_EVENT_COMMAND_ADAPTER: &str = "/backend/v3/api/cms/outbox_events/{eventCommand}";

pub fn gateway_mount(state: sdkwork_cms_http_handlers::AppState) -> axum::Router {
    use axum::routing::{delete, get, patch, post, put};
    use sdkwork_cms_http_handlers::handlers;

    axum::Router::new()
        .route(
            paths::SITES,
            get(handlers::list_sites).post(handlers::create_site),
        )
        .route(
            paths::SITE_BY_ID,
            get(handlers::retrieve_site)
                .patch(handlers::update_site)
                .delete(handlers::delete_site),
        )
        .route(
            paths::SITE_CHANNELS,
            get(handlers::list_channels).post(handlers::create_channel),
        )
        .route(
            paths::CHANNEL_BY_ID,
            patch(handlers::update_channel).delete(handlers::delete_channel),
        )
        .route(
            paths::SITE_CONTENT_TYPES,
            get(handlers::list_content_types).post(handlers::create_content_type),
        )
        .route(
            paths::CONTENT_TYPE_BY_ID,
            get(handlers::retrieve_content_type)
                .patch(handlers::update_content_type)
                .delete(handlers::delete_content_type),
        )
        .route(
            paths::CONTENT_TYPE_FIELDS,
            get(handlers::list_content_fields).post(handlers::create_content_field),
        )
        .route(
            paths::CONTENT_FIELD_BY_ID,
            patch(handlers::update_content_field).delete(handlers::delete_content_field),
        )
        .route(
            paths::SITE_TAXONOMIES,
            get(handlers::list_taxonomies).post(handlers::create_taxonomy),
        )
        .route(
            paths::TAXONOMY_BY_ID,
            patch(handlers::update_taxonomy).delete(handlers::delete_taxonomy),
        )
        .route(
            paths::TAXONOMY_TERMS,
            get(handlers::list_taxonomy_terms).post(handlers::create_taxonomy_term),
        )
        .route(
            paths::TAXONOMY_TERM_BY_ID,
            patch(handlers::update_taxonomy_term).delete(handlers::delete_taxonomy_term),
        )
        .route(
            paths::ENTRIES,
            get(handlers::list_entries).post(handlers::create_entry),
        )
        .route(
            paths::ENTRY_BY_ID,
            get(handlers::retrieve_entry)
                .patch(handlers::update_entry)
                .delete(handlers::delete_entry)
                .post(dispatch_entry_command),
        )
        .route(paths::ENTRY_BODY, put(handlers::replace_entry_body))
        .route(paths::ENTRY_FIELDS, put(handlers::replace_entry_fields))
        .route(
            paths::ENTRY_MEDIA,
            get(handlers::list_entry_media).post(handlers::attach_entry_media),
        )
        .route(
            paths::ENTRY_MEDIA_BY_ID,
            delete(handlers::delete_entry_media),
        )
        .route(paths::ENTRY_TERMS, put(handlers::replace_entry_terms))
        .route(paths::ENTRY_VERSIONS, get(handlers::list_entry_versions))
        .route(
            paths::PAGES,
            get(handlers::list_pages).post(handlers::create_page),
        )
        .route(
            paths::PAGE_BY_ID,
            get(handlers::retrieve_page)
                .patch(handlers::update_page)
                .delete(handlers::delete_page)
                .post(dispatch_page_command),
        )
        .route(paths::PAGE_BLOCKS, put(handlers::replace_page_blocks))
        .route(
            paths::FEEDS,
            get(handlers::list_feeds).post(handlers::create_feed),
        )
        .route(
            paths::FEED_BY_ID,
            get(handlers::retrieve_feed)
                .patch(handlers::update_feed)
                .delete(handlers::delete_feed)
                .post(dispatch_feed_command),
        )
        .route(
            paths::FEED_RULES,
            get(handlers::list_feed_rules).post(handlers::create_feed_rule),
        )
        .route(
            paths::FEED_RULE_BY_ID,
            patch(handlers::update_feed_rule).delete(handlers::delete_feed_rule),
        )
        .route(
            paths::FEED_ITEMS,
            get(handlers::list_feed_items).put(handlers::upsert_feed_items),
        )
        .route(paths::FEED_ITEM_BY_ID, delete(handlers::delete_feed_item))
        .route(
            paths::FEED_SNAPSHOT_BY_ID,
            get(handlers::retrieve_feed_snapshot),
        )
        .route(paths::AUDIT_LOGS, get(handlers::list_audit_logs))
        .route(paths::OUTBOX_EVENTS, get(handlers::list_outbox_events))
        .route(
            OUTBOX_EVENT_COMMAND_ADAPTER,
            post(dispatch_outbox_event_command),
        )
        .with_state(state)
}

fn parse_command_segment(segment: &str) -> Result<(i64, &str), axum::http::StatusCode> {
    let (raw_id, action) = segment
        .rsplit_once(':')
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    if raw_id.is_empty() || action.is_empty() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }
    let id = raw_id
        .parse::<i64>()
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    Ok((id, action))
}

fn invalid_json(error: serde_json::Error) -> axum::response::Response {
    use axum::response::IntoResponse;

    (
        axum::http::StatusCode::BAD_REQUEST,
        axum::Json(serde_json::json!({
            "ok": false,
            "error": { "detail": format!("invalid command request: {error}") }
        })),
    )
        .into_response()
}

async fn dispatch_entry_command(
    axum::extract::State(state): axum::extract::State<sdkwork_cms_http_handlers::AppState>,
    context: sdkwork_cms_http_handlers::CmsHttpRequestContext,
    axum::extract::Path(segment): axum::extract::Path<String>,
    axum::Json(body): axum::Json<serde_json::Value>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    use sdkwork_cms_http_handlers::handlers;

    let (entry_id, action) = match parse_command_segment(&segment) {
        Ok(command) => command,
        Err(status) => return status.into_response(),
    };
    match action {
        "publish" => match serde_json::from_value::<handlers::PublishRequest>(body) {
            Ok(request) => handlers::publish_entry(
                axum::extract::State(state),
                context,
                axum::extract::Path(entry_id),
                axum::Json(request),
            )
            .await
            .into_response(),
            Err(error) => invalid_json(error),
        },
        "unpublish" => match serde_json::from_value::<handlers::PublishRequest>(body) {
            Ok(request) => handlers::unpublish_entry(
                axum::extract::State(state),
                context,
                axum::extract::Path(entry_id),
                axum::Json(request),
            )
            .await
            .into_response(),
            Err(error) => invalid_json(error),
        },
        "rollback" => match serde_json::from_value::<handlers::RollbackRequest>(body) {
            Ok(request) => handlers::rollback_entry(
                axum::extract::State(state),
                context,
                axum::extract::Path(entry_id),
                axum::Json(request),
            )
            .await
            .into_response(),
            Err(error) => invalid_json(error),
        },
        "schedule" => match serde_json::from_value::<handlers::ScheduleRequest>(body) {
            Ok(request) => handlers::schedule_entry(
                axum::extract::State(state),
                context,
                axum::extract::Path(entry_id),
                axum::Json(request),
            )
            .await
            .into_response(),
            Err(error) => invalid_json(error),
        },
        _ => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

async fn dispatch_page_command(
    axum::extract::State(state): axum::extract::State<sdkwork_cms_http_handlers::AppState>,
    context: sdkwork_cms_http_handlers::CmsHttpRequestContext,
    axum::extract::Path(segment): axum::extract::Path<String>,
    axum::Json(body): axum::Json<serde_json::Value>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    use sdkwork_cms_http_handlers::handlers;

    let (page_id, action) = match parse_command_segment(&segment) {
        Ok(command) => command,
        Err(status) => return status.into_response(),
    };
    if action != "publish" {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    }
    match serde_json::from_value::<handlers::PublishRequest>(body) {
        Ok(request) => handlers::publish_page(
            axum::extract::State(state),
            context,
            axum::extract::Path(page_id),
            axum::Json(request),
        )
        .await
        .into_response(),
        Err(error) => invalid_json(error),
    }
}

async fn dispatch_feed_command(
    axum::extract::State(state): axum::extract::State<sdkwork_cms_http_handlers::AppState>,
    context: sdkwork_cms_http_handlers::CmsHttpRequestContext,
    axum::extract::Path(segment): axum::extract::Path<String>,
    axum::Json(body): axum::Json<serde_json::Value>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    use sdkwork_cms_http_handlers::handlers;

    let (feed_id, action) = match parse_command_segment(&segment) {
        Ok(command) => command,
        Err(status) => return status.into_response(),
    };
    if action != "publish" {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    }
    match serde_json::from_value::<handlers::PublishRequest>(body) {
        Ok(request) => handlers::publish_feed(
            axum::extract::State(state),
            context,
            axum::extract::Path(feed_id),
            axum::Json(request),
        )
        .await
        .into_response(),
        Err(error) => invalid_json(error),
    }
}

async fn dispatch_outbox_event_command(
    axum::extract::State(state): axum::extract::State<sdkwork_cms_http_handlers::AppState>,
    context: sdkwork_cms_http_handlers::CmsHttpRequestContext,
    axum::extract::Path(segment): axum::extract::Path<String>,
    axum::Json(body): axum::Json<serde_json::Value>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    use sdkwork_cms_http_handlers::handlers;

    let (event_id, action) = match parse_command_segment(&segment) {
        Ok(command) => command,
        Err(status) => return status.into_response(),
    };
    if action != "retry" {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    }
    match serde_json::from_value::<handlers::RetryOutboxEventRequest>(body) {
        Ok(request) => handlers::retry_outbox_event(
            axum::extract::State(state),
            context,
            axum::extract::Path(event_id),
            axum::Json(request),
        )
        .await
        .into_response(),
        Err(error) => invalid_json(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_command_segment() {
        assert_eq!(parse_command_segment("42:publish"), Ok((42, "publish")));
        assert_eq!(parse_command_segment("42:rollback"), Ok((42, "rollback")));
    }

    #[test]
    fn rejects_non_command_and_invalid_ids() {
        assert_eq!(
            parse_command_segment("42"),
            Err(axum::http::StatusCode::NOT_FOUND)
        );
        assert_eq!(
            parse_command_segment("entry:publish"),
            Err(axum::http::StatusCode::BAD_REQUEST)
        );
    }
}
