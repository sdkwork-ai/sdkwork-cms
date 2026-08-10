use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;

use crate::{AppState, CmsHttpRequestContext};
use sdkwork_content_cms_service::domain::*;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Deserialize)]
pub struct SiteCreateRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub default_locale: Option<String>,
    pub settings_json: Option<String>,
}

#[derive(Deserialize)]
pub struct SiteUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_locale: Option<String>,
    pub settings_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct ChannelCreateRequest {
    pub code: String,
    pub name: String,
    pub channel_kind: Option<String>,
}

#[derive(Deserialize)]
pub struct ChannelUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub channel_kind: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct ContentTypeCreateRequest {
    pub code: String,
    pub name: String,
    pub content_kind: Option<String>,
}

#[derive(Deserialize)]
pub struct ContentTypeUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub content_kind: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct ContentFieldCreateRequest {
    pub code: String,
    pub name: String,
    pub field_kind: String,
}

#[derive(Deserialize)]
pub struct ContentFieldUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub field_kind: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct TaxonomyCreateRequest {
    pub code: String,
    pub name: String,
    pub taxonomy_kind: Option<String>,
}

#[derive(Deserialize)]
pub struct TaxonomyUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub taxonomy_kind: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct TaxonomyTermCreateRequest {
    pub code: String,
    pub name: String,
    pub slug: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct TaxonomyTermUpdateRequest {
    pub taxonomy_id: i64,
    pub code: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub parent_id: Option<i64>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct EntryCreateRequest {
    pub site_id: i64,
    pub content_type_id: i64,
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
}

#[derive(Deserialize)]
pub struct EntryUpdateRequest {
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct EntryBodyRequest {
    pub locale: String,
    pub body_format: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub block_tree_json: String,
}

#[derive(Deserialize)]
pub struct EntryFieldsRequest {
    pub locale: String,
    pub fields_json: String,
}

#[derive(Deserialize)]
pub struct EntryMediaRequest {
    pub media_role: String,
    pub drive_node_id: Option<String>,
    pub drive_uri: Option<String>,
}

#[derive(Deserialize)]
pub struct EntryTermsRequest {
    pub term_ids: Vec<i64>,
}

#[derive(Deserialize)]
pub struct PublishRequest {
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub note: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct RollbackRequest {
    pub target_version_id: i64,
    pub note: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct ScheduleRequest {
    pub scheduled_publish_at: Option<String>,
    pub scheduled_unpublish_at: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct PageCreateRequest {
    pub site_id: i64,
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub path: String,
    pub slug: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct PageUpdateRequest {
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub path: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct PageBlocksRequest {
    pub blocks_json: String,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct FeedCreateRequest {
    pub site_id: i64,
    pub channel_id: Option<i64>,
    pub code: String,
    pub name: String,
    pub feed_kind: Option<String>,
    pub locale: Option<String>,
}

#[derive(Deserialize)]
pub struct FeedUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub feed_kind: Option<String>,
    pub locale: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct FeedRuleRequest {
    pub feed_id: Option<i64>,
    pub rule_kind: String,
    pub condition_json: String,
    pub sort_json: String,
    pub limit_count: u32,
    pub enabled: bool,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct FeedItemsRequest {
    pub items_json: String,
    pub version: Option<i64>,
}

#[derive(Deserialize)]
pub struct RetryOutboxEventRequest {
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct DeliveryBootstrapParams {
    pub channel_code: Option<String>,
    pub locale: Option<String>,
}

#[derive(Deserialize)]
pub struct DeliveryResolveParams {
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub slug: Option<String>,
    pub path: Option<String>,
    pub preview_token: Option<String>,
}

#[derive(Deserialize)]
pub struct DeliveryFeedItemsParams {
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Deserialize)]
pub struct CreateFavoriteRequest {
    pub target_type: String,
    pub target_id: Option<i64>,
    pub target_uuid: Option<String>,
    pub target_url: Option<String>,
    pub favorite_type: String,
    pub title: String,
    pub summary: String,
    pub source_display_name: String,
    pub media: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct FavoriteListParams {
    pub favorite_type: Option<String>,
    pub q: Option<String>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

// ============ Backend API Handlers ============

pub async fn list_sites(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListSitesQuery {
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_sites(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(|s| serde_json::json!({
                    "id": s.id.to_string(),
                    "uuid": s.uuid,
                    "tenantId": s.tenant_id.to_string(),
                    "organizationId": s.organization_id.to_string(),
                    "code": s.code,
                    "name": s.name,
                    "defaultLocale": s.default_locale,
                    "settingsJson": s.settings_json,
                    "status": s.status,
                    "version": s.version.to_string(),
                })).collect::<Vec<_>>(),
                "nextCursor": page.next_cursor,
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_site(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Json(req): Json<SiteCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = SiteCommand {
        code: Some(req.code),
        name: Some(req.name),
        description: req.description,
        default_locale: req.default_locale,
        settings_json: req.settings_json,
        version: None,
    };
    match state.service.create_site(&ctx, command).await {
        Ok(site) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "ok": true,
                "data": {
                    "id": site.id.to_string(),
                    "uuid": site.uuid,
                    "code": site.code,
                    "name": site.name,
                }
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn retrieve_site(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.retrieve_site(&ctx, site_id).await {
        Ok(site) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "id": site.id.to_string(),
                "uuid": site.uuid,
                "code": site.code,
                "name": site.name,
                "defaultLocale": site.default_locale,
                "status": site.status,
                "version": site.version.to_string(),
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn update_site(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
    Json(req): Json<SiteUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = SiteCommand {
        code: req.code,
        name: req.name,
        description: req.description,
        default_locale: req.default_locale,
        settings_json: req.settings_json,
        version: req.version,
    };
    match state.service.update_site(&ctx, site_id, command).await {
        Ok(site) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "id": site.id.to_string(),
                "code": site.code,
                "name": site.name,
                "version": site.version.to_string(),
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_site(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_site(&ctx, site_id).await {
        Ok(result) => Json(
            serde_json::json!({"ok": result.ok, "resourceId": result.resource_id.map(|id| id.to_string())}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_channels(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListBySiteQuery {
        site_id,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_channels(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(|c| serde_json::json!({
                    "id": c.id.to_string(),
                    "siteId": c.site_id.to_string(),
                    "code": c.code,
                    "name": c.name,
                    "channelKind": c.channel_kind,
                    "status": c.status,
                })).collect::<Vec<_>>(),
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_channel(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
    Json(req): Json<ChannelCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = ChannelCommand {
        site_id,
        code: Some(req.code),
        name: Some(req.name),
        channel_kind: req.channel_kind,
        delivery_config_json: None,
        version: None,
    };
    match state.service.create_channel(&ctx, command).await {
        Ok(ch) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": ch.id.to_string(), "code": ch.code, "name": ch.name}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn update_channel(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(channel_id): Path<i64>,
    Json(req): Json<ChannelUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = ChannelCommand {
        site_id: 0,
        code: req.code,
        name: req.name,
        channel_kind: req.channel_kind,
        delivery_config_json: None,
        version: req.version,
    };
    match state
        .service
        .update_channel(&ctx, channel_id, command)
        .await
    {
        Ok(ch) => Json(
            serde_json::json!({"ok": true, "data": {"id": ch.id.to_string(), "code": ch.code, "name": ch.name}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_channel(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(channel_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_channel(&ctx, channel_id).await {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_content_types(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListBySiteQuery {
        site_id,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_content_types(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(|ct| serde_json::json!({
                    "id": ct.id.to_string(), "siteId": ct.site_id.to_string(), "code": ct.code, "name": ct.name,
                    "contentKind": ct.content_kind, "schemaVersion": ct.schema_version.to_string(), "status": ct.status,
                })).collect::<Vec<_>>(),
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_content_type(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
    Json(req): Json<ContentTypeCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = ContentTypeCommand {
        site_id,
        code: Some(req.code),
        name: Some(req.name),
        content_kind: req.content_kind,
        settings_json: None,
        version: None,
    };
    match state.service.create_content_type(&ctx, command).await {
        Ok(ct) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": ct.id.to_string(), "code": ct.code, "name": ct.name}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn retrieve_content_type(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(content_type_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state
        .service
        .retrieve_content_type(&ctx, content_type_id)
        .await
    {
        Ok(ct) => Json(
            serde_json::json!({"ok": true, "data": {"id": ct.id.to_string(), "code": ct.code, "name": ct.name, "contentKind": ct.content_kind}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn update_content_type(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(content_type_id): Path<i64>,
    Json(req): Json<ContentTypeUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = ContentTypeCommand {
        site_id: 0,
        code: req.code,
        name: req.name,
        content_kind: req.content_kind,
        settings_json: None,
        version: req.version,
    };
    match state
        .service
        .update_content_type(&ctx, content_type_id, command)
        .await
    {
        Ok(ct) => Json(
            serde_json::json!({"ok": true, "data": {"id": ct.id.to_string(), "code": ct.code, "name": ct.name}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_content_type(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(content_type_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state
        .service
        .delete_content_type(&ctx, content_type_id)
        .await
    {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_content_fields(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(content_type_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListContentFieldsQuery {
        content_type_id,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_content_fields(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(|f| serde_json::json!({
                    "id": f.id.to_string(), "contentTypeId": f.content_type_id.to_string(), "code": f.code, "name": f.name,
                    "fieldKind": f.field_kind, "required": f.required, "searchable": f.searchable,
                })).collect::<Vec<_>>(),
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_content_field(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(content_type_id): Path<i64>,
    Json(req): Json<ContentFieldCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = ContentFieldCommand {
        content_type_id,
        code: Some(req.code),
        name: Some(req.name),
        field_kind: Some(req.field_kind),
        validation_json: None,
        options_json: None,
        ui_json: None,
        version: None,
    };
    match state.service.create_content_field(&ctx, command).await {
        Ok(f) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": f.id.to_string(), "code": f.code, "name": f.name}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn update_content_field(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(field_id): Path<i64>,
    Json(req): Json<ContentFieldUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = ContentFieldCommand {
        content_type_id: 0,
        code: req.code,
        name: req.name,
        field_kind: req.field_kind,
        validation_json: None,
        options_json: None,
        ui_json: None,
        version: req.version,
    };
    match state
        .service
        .update_content_field(&ctx, field_id, command)
        .await
    {
        Ok(f) => Json(
            serde_json::json!({"ok": true, "data": {"id": f.id.to_string(), "code": f.code, "name": f.name}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_content_field(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(field_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_content_field(&ctx, field_id).await {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_taxonomies(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListBySiteQuery {
        site_id,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_taxonomies(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(|t| serde_json::json!({
                    "id": t.id.to_string(), "siteId": t.site_id.to_string(), "code": t.code, "name": t.name,
                    "taxonomyKind": t.taxonomy_kind, "status": t.status,
                })).collect::<Vec<_>>(),
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_taxonomy(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_id): Path<i64>,
    Json(req): Json<TaxonomyCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = TaxonomyCommand {
        site_id,
        code: Some(req.code),
        name: Some(req.name),
        taxonomy_kind: req.taxonomy_kind,
        settings_json: None,
        version: None,
    };
    match state.service.create_taxonomy(&ctx, command).await {
        Ok(t) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": t.id.to_string(), "code": t.code, "name": t.name}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn update_taxonomy(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(taxonomy_id): Path<i64>,
    Json(req): Json<TaxonomyUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = TaxonomyCommand {
        site_id: 0,
        code: req.code,
        name: req.name,
        taxonomy_kind: req.taxonomy_kind,
        settings_json: None,
        version: req.version,
    };
    match state
        .service
        .update_taxonomy(&ctx, taxonomy_id, command)
        .await
    {
        Ok(t) => Json(
            serde_json::json!({"ok": true, "data": {"id": t.id.to_string(), "code": t.code, "name": t.name}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_taxonomy(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(taxonomy_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_taxonomy(&ctx, taxonomy_id).await {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_taxonomy_terms(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(taxonomy_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListTaxonomyTermsQuery {
        taxonomy_id,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_taxonomy_terms(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(|t| serde_json::json!({
                    "id": t.id.to_string(), "taxonomyId": t.taxonomy_id.to_string(), "parentId": t.parent_id.map(|id| id.to_string()),
                    "code": t.code, "slug": t.slug, "name": t.name, "path": t.path, "status": t.status,
                })).collect::<Vec<_>>(),
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_taxonomy_term(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(taxonomy_id): Path<i64>,
    Json(req): Json<TaxonomyTermCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = TaxonomyTermCommand {
        taxonomy_id,
        parent_id: req.parent_id,
        code: Some(req.code),
        slug: req.slug,
        name: Some(req.name),
        version: None,
    };
    match state.service.create_taxonomy_term(&ctx, command).await {
        Ok(t) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": t.id.to_string(), "code": t.code, "name": t.name, "path": t.path}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn update_taxonomy_term(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(term_id): Path<i64>,
    Json(req): Json<TaxonomyTermUpdateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = TaxonomyTermCommand {
        taxonomy_id: req.taxonomy_id,
        parent_id: req.parent_id,
        code: req.code,
        slug: req.slug,
        name: req.name,
        version: req.version,
    };
    match state
        .service
        .update_taxonomy_term(&ctx, term_id, command)
        .await
    {
        Ok(term) => (
            StatusCode::OK,
            Json(
                serde_json::json!({"ok": true, "data": {"id": term.id.to_string(), "taxonomyId": term.taxonomy_id.to_string(), "code": term.code, "name": term.name, "slug": term.slug}}),
            ),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": error.to_string()}})),
        ),
    }
}

pub async fn delete_taxonomy_term(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(term_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_taxonomy_term(&ctx, term_id).await {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

// ============ Entry Handlers ============

pub async fn list_entries(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListEntriesQuery {
        site_id: None,
        content_type_id: None,
        channel_id: None,
        locale: None,
        entry_status: None,
        publication_status: None,
        author_user_id: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_entries(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(|e| serde_json::json!({
                    "id": e.id.to_string(), "uuid": e.uuid, "siteId": e.site_id.to_string(),
                    "contentTypeId": e.content_type_id.to_string(), "channelId": e.channel_id.map(|id| id.to_string()),
                    "locale": e.locale, "title": e.title, "slug": e.slug, "summary": e.summary,
                    "entryStatus": format!("{:?}", e.entry_status).to_lowercase(),
                    "publicationStatus": format!("{:?}", e.publication_status).to_lowercase(),
                    "publishedAt": e.published_at, "version": e.version.to_string(),
                })).collect::<Vec<_>>(),
                "nextCursor": page.next_cursor,
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Json(req): Json<EntryCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = EntryCommand {
        site_id: req.site_id,
        content_type_id: req.content_type_id,
        channel_id: req.channel_id,
        locale: req.locale.unwrap_or_else(|| "zh-CN".to_string()),
        title: req.title,
        slug: req.slug,
        summary: req.summary,
        seo_json: None,
        version: None,
    };
    match state.service.create_entry(&ctx, command).await {
        Ok(entry) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title, "slug": entry.slug}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn retrieve_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.retrieve_entry(&ctx, entry_id).await {
        Ok(entry) => Json(serde_json::json!({"ok": true, "data": {
            "id": entry.id.to_string(), "uuid": entry.uuid, "title": entry.title, "slug": entry.slug,
            "locale": entry.locale, "entryStatus": format!("{:?}", entry.entry_status).to_lowercase(),
            "publicationStatus": format!("{:?}", entry.publication_status).to_lowercase(),
            "version": entry.version.to_string(),
        }})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn update_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<EntryUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = EntryCommand {
        site_id: 0,
        content_type_id: 0,
        channel_id: req.channel_id,
        locale: req.locale.unwrap_or_default(),
        title: req.title.unwrap_or_default(),
        slug: req.slug.unwrap_or_default(),
        summary: req.summary,
        seo_json: None,
        version: req.version,
    };
    match state.service.update_entry(&ctx, entry_id, command).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title, "version": entry.version.to_string()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_entry(&ctx, entry_id).await {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn replace_entry_body(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<EntryBodyRequest>,
) -> Json<serde_json::Value> {
    let command = EntryBodyCommand {
        entry_id,
        locale: req.locale,
        body_format: req.body_format,
        body_text: req.body_text,
        body_html: req.body_html,
        block_tree_json: req.block_tree_json,
        version: None,
    };
    match state.service.replace_entry_body(&ctx, command).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn replace_entry_fields(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<EntryFieldsRequest>,
) -> Json<serde_json::Value> {
    let command = EntryFieldsCommand {
        entry_id,
        locale: req.locale,
        fields_json: req.fields_json,
        version: None,
    };
    match state.service.replace_entry_fields(&ctx, command).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_entry_media(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListEntryMediaQuery {
        entry_id,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_entry_media(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|m| serde_json::json!({"id": m.id.to_string(), "role": m.role})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn attach_entry_media(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<EntryMediaRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = EntryMediaCommand {
        entry_id,
        field_id: None,
        media_role: req.media_role,
        drive_space_id: None,
        drive_node_id: req.drive_node_id,
        drive_uri: req.drive_uri,
        media_resource_id: None,
        media_snapshot_json: "{}".to_string(),
        alt_text: None,
        caption: None,
    };
    match state.service.attach_entry_media(&ctx, command).await {
        Ok(media) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": media.id.to_string(), "role": media.role}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn replace_entry_terms(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<EntryTermsRequest>,
) -> Json<serde_json::Value> {
    let command = ReplaceEntryTermsCommand {
        entry_id,
        term_ids: req.term_ids,
        version: None,
    };
    match state.service.replace_entry_terms(&ctx, command).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_entry_versions(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListEntryVersionsQuery {
        entry_id,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_entry_versions(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|v| serde_json::json!({"id": v.id.to_string(), "entryId": v.entry_id.to_string(), "versionNo": v.version_no.to_string(), "versionKind": v.version_kind})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn publish_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<PublishRequest>,
) -> Json<serde_json::Value> {
    let command = PublishCommand {
        owner_type: "entry".to_string(),
        owner_id: entry_id,
        channel_id: req.channel_id,
        locale: req.locale,
        note: req.note,
        version: req.version,
    };
    match state.service.publish_entry(&ctx, command).await {
        Ok(snapshot) => Json(
            serde_json::json!({"ok": true, "data": {"id": snapshot.id.to_string(), "ownerType": snapshot.owner_type, "ownerId": snapshot.owner_id.to_string(), "status": snapshot.status}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn unpublish_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<PublishRequest>,
) -> Json<serde_json::Value> {
    let command = PublishCommand {
        owner_type: "entry".to_string(),
        owner_id: entry_id,
        channel_id: req.channel_id,
        locale: req.locale,
        note: req.note,
        version: req.version,
    };
    match state.service.unpublish_entry(&ctx, command).await {
        Ok(snapshot) => Json(
            serde_json::json!({"ok": true, "data": {"id": snapshot.id.to_string(), "status": snapshot.status}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn rollback_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<RollbackRequest>,
) -> Json<serde_json::Value> {
    let command = RollbackCommand {
        owner_type: "entry".to_string(),
        owner_id: entry_id,
        target_version_id: req.target_version_id,
        note: req.note,
        version: req.version,
    };
    match state.service.rollback_entry(&ctx, command).await {
        Ok(snapshot) => Json(
            serde_json::json!({"ok": true, "data": {"id": snapshot.id.to_string(), "status": snapshot.status}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn schedule_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Json(req): Json<ScheduleRequest>,
) -> Json<serde_json::Value> {
    let command = ScheduleCommand {
        entry_id,
        scheduled_publish_at: req.scheduled_publish_at,
        scheduled_unpublish_at: req.scheduled_unpublish_at,
        version: req.version,
    };
    match state.service.schedule_entry(&ctx, command).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

// ============ Page Handlers ============

pub async fn list_pages(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListPagesQuery {
        site_id: None,
        channel_id: None,
        locale: None,
        status: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_pages(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|p| serde_json::json!({"id": p.id.to_string(), "siteId": p.site_id.to_string(), "path": p.path, "title": p.title, "locale": p.locale})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_page(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Json(req): Json<PageCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = PageCommand {
        site_id: req.site_id,
        channel_id: req.channel_id,
        locale: req.locale.unwrap_or_else(|| "zh-CN".to_string()),
        path: req.path,
        slug: req.slug,
        title: req.title,
        seo_json: None,
        settings_json: None,
        version: None,
    };
    match state.service.create_page(&ctx, command).await {
        Ok(page) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": page.id.to_string(), "title": page.title, "path": page.path}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn retrieve_page(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(page_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.retrieve_page(&ctx, page_id).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"id": page.id.to_string(), "title": page.title, "path": page.path, "locale": page.locale}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn update_page(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(page_id): Path<i64>,
    Json(req): Json<PageUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = PageCommand {
        site_id: 0,
        channel_id: req.channel_id,
        locale: req.locale.unwrap_or_default(),
        path: req.path.unwrap_or_default(),
        slug: req.slug.unwrap_or_default(),
        title: req.title.unwrap_or_default(),
        seo_json: None,
        settings_json: None,
        version: req.version,
    };
    match state.service.update_page(&ctx, page_id, command).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"id": page.id.to_string(), "title": page.title}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_page(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(page_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_page(&ctx, page_id).await {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn replace_page_blocks(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(page_id): Path<i64>,
    Json(req): Json<PageBlocksRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = PageBlocksCommand {
        page_id,
        blocks_json: req.blocks_json,
        version: req.version,
    };
    match state.service.replace_page_blocks(&ctx, command).await {
        Ok(page) => (
            StatusCode::OK,
            Json(
                serde_json::json!({"ok": true, "data": {"id": page.id.to_string(), "title": page.title, "path": page.path}}),
            ),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": error.to_string()}})),
        ),
    }
}

pub async fn publish_page(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(page_id): Path<i64>,
    Json(req): Json<PublishRequest>,
) -> Json<serde_json::Value> {
    let command = PublishCommand {
        owner_type: "page".to_string(),
        owner_id: page_id,
        channel_id: req.channel_id,
        locale: req.locale,
        note: req.note,
        version: req.version,
    };
    match state.service.publish_page(&ctx, command).await {
        Ok(snapshot) => Json(
            serde_json::json!({"ok": true, "data": {"id": snapshot.id.to_string(), "status": snapshot.status}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

// ============ Feed Handlers ============

pub async fn list_feeds(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListFeedsQuery {
        site_id: None,
        channel_id: None,
        locale: None,
        status: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_feeds(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|f| serde_json::json!({"id": f.id.to_string(), "code": f.code, "name": f.name, "feedKind": f.feed_kind, "locale": f.locale})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_feed(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Json(req): Json<FeedCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = FeedCommand {
        site_id: req.site_id,
        channel_id: req.channel_id,
        code: Some(req.code),
        name: Some(req.name),
        feed_kind: req.feed_kind,
        locale: req.locale,
        rule_config_json: None,
        version: None,
    };
    match state.service.create_feed(&ctx, command).await {
        Ok(feed) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": feed.id.to_string(), "code": feed.code, "name": feed.name}}),
            ),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn retrieve_feed(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.retrieve_feed(&ctx, feed_id).await {
        Ok(feed) => Json(
            serde_json::json!({"ok": true, "data": {"id": feed.id.to_string(), "code": feed.code, "name": feed.name, "feedKind": feed.feed_kind}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn update_feed(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
    Json(req): Json<FeedUpdateRequest>,
) -> Json<serde_json::Value> {
    let command = FeedCommand {
        site_id: 0,
        channel_id: None,
        code: req.code,
        name: req.name,
        feed_kind: req.feed_kind,
        locale: req.locale,
        rule_config_json: None,
        version: req.version,
    };
    match state.service.update_feed(&ctx, feed_id, command).await {
        Ok(feed) => Json(
            serde_json::json!({"ok": true, "data": {"id": feed.id.to_string(), "code": feed.code, "name": feed.name}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_feed(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
) -> Json<serde_json::Value> {
    match state.service.delete_feed(&ctx, feed_id).await {
        Ok(result) => Json(serde_json::json!({"ok": result.ok})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_feed_rules(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListFeedRulesQuery {
        feed_id,
        enabled: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_feed_rules(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|r| serde_json::json!({"id": r.id.to_string(), "feedId": r.feed_id.to_string(), "ruleKind": r.rule_kind, "enabled": r.enabled})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn create_feed_rule(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
    Json(req): Json<FeedRuleRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = FeedRuleCommand {
        feed_id,
        rule_kind: req.rule_kind,
        condition_json: req.condition_json,
        sort_json: req.sort_json,
        limit_count: req.limit_count,
        enabled: req.enabled,
        version: req.version,
    };
    match state.service.create_feed_rule(&ctx, command).await {
        Ok(rule) => (
            StatusCode::CREATED,
            Json(
                serde_json::json!({"ok": true, "data": {"id": rule.id.to_string(), "feedId": rule.feed_id.to_string(), "ruleKind": rule.rule_kind, "enabled": rule.enabled}}),
            ),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": error.to_string()}})),
        ),
    }
}

pub async fn delete_entry_media(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path((_entry_id, media_id)): Path<(i64, i64)>,
) -> StatusCode {
    match state.service.delete_entry_media(&ctx, media_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn update_feed_rule(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(rule_id): Path<i64>,
    Json(req): Json<FeedRuleRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = FeedRuleCommand {
        feed_id: req.feed_id.unwrap_or_default(),
        rule_kind: req.rule_kind,
        condition_json: req.condition_json,
        sort_json: req.sort_json,
        limit_count: req.limit_count,
        enabled: req.enabled,
        version: req.version,
    };
    match state.service.update_feed_rule(&ctx, rule_id, command).await {
        Ok(rule) => (
            StatusCode::OK,
            Json(
                serde_json::json!({"ok": true, "data": {"id": rule.id.to_string(), "feedId": rule.feed_id.to_string(), "ruleKind": rule.rule_kind, "enabled": rule.enabled}}),
            ),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": error.to_string()}})),
        ),
    }
}

pub async fn delete_feed_rule(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(rule_id): Path<i64>,
) -> StatusCode {
    match state.service.delete_feed_rule(&ctx, rule_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn upsert_feed_items(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
    Json(req): Json<FeedItemsRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = FeedItemsCommand {
        feed_id,
        items_json: req.items_json,
        version: req.version,
    };
    match state.service.upsert_feed_items(&ctx, command).await {
        Ok(page) => (
            StatusCode::OK,
            Json(
                serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|item| serde_json::json!({"id": item.id.to_string(), "feedId": item.feed_id.to_string(), "itemKind": item.item_kind})).collect::<Vec<_>>()}}),
            ),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": error.to_string()}})),
        ),
    }
}

pub async fn delete_feed_item(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(item_id): Path<i64>,
) -> StatusCode {
    match state.service.delete_feed_item(&ctx, item_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn retry_outbox_event(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(event_id): Path<i64>,
    Json(req): Json<RetryOutboxEventRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = RetryOutboxEventCommand {
        event_id,
        reason: req.reason,
    };
    match state.service.retry_outbox_event(&ctx, command).await {
        Ok(result) => (
            StatusCode::OK,
            Json(serde_json::json!({"ok": true, "data": {"accepted": result.ok}})),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": error.to_string()}})),
        ),
    }
}

pub async fn list_feed_items(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListFeedItemsQuery {
        feed_id,
        status: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_feed_items(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|i| serde_json::json!({"id": i.id.to_string(), "feedId": i.feed_id.to_string(), "entryId": i.entry_id.map(|id| id.to_string()), "itemKind": i.item_kind, "pinned": i.pinned})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn publish_feed(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_id): Path<i64>,
    Json(req): Json<PublishRequest>,
) -> Json<serde_json::Value> {
    let command = PublishCommand {
        owner_type: "feed".to_string(),
        owner_id: feed_id,
        channel_id: req.channel_id,
        locale: req.locale,
        note: req.note,
        version: req.version,
    };
    match state.service.publish_feed(&ctx, command).await {
        Ok(snapshot) => Json(
            serde_json::json!({"ok": true, "data": {"id": snapshot.id.to_string(), "status": snapshot.status}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn retrieve_feed_snapshot(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path((_feed_id, snapshot_id)): Path<(i64, i64)>,
) -> Json<serde_json::Value> {
    match state
        .service
        .retrieve_feed_snapshot(&ctx, snapshot_id)
        .await
    {
        Ok(snapshot) => Json(
            serde_json::json!({"ok": true, "data": {"id": snapshot.id.to_string(), "feedId": snapshot.feed_id.to_string(), "snapshotVersion": snapshot.snapshot_version.to_string(), "itemCount": snapshot.item_count}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

// ============ Governance Handlers ============

pub async fn list_audit_logs(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListAuditLogsQuery {
        site_id: None,
        resource_type: None,
        resource_id: None,
        actor_user_id: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_audit_logs(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|l| serde_json::json!({"id": l.id.to_string(), "action": l.action, "resourceType": l.resource_type, "resourceId": l.resource_id.map(|id| id.to_string()), "actorUserId": l.actor_user_id.to_string(), "createdAt": l.created_at})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn list_outbox_events(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = ListOutboxEventsQuery {
        aggregate_type: None,
        aggregate_id: None,
        status: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_outbox_events(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|e| serde_json::json!({"id": e.id.to_string(), "aggregateType": e.aggregate_type, "aggregateId": e.aggregate_id.to_string(), "eventType": e.event_type, "status": e.status})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

// ============ Delivery Handlers (App API) ============

pub async fn delivery_bootstrap(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_code): Path<String>,
    Query(params): Query<DeliveryBootstrapParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryBootstrapQuery {
        site_code,
        channel_code: params.channel_code,
        locale: params.locale,
    };
    match state.service.delivery_bootstrap(&ctx, query).await {
        Ok(bootstrap) => Json(serde_json::json!({"ok": true, "data": {
            "site": {"id": bootstrap.site.id.to_string(), "code": bootstrap.site.code, "name": bootstrap.site.name, "defaultLocale": bootstrap.site.default_locale},
            "channels": bootstrap.channels.iter().map(|c| serde_json::json!({"id": c.id.to_string(), "code": c.code, "name": c.name, "channelKind": c.channel_kind})).collect::<Vec<_>>(),
        }})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delivery_resolve_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_code): Path<String>,
    Query(params): Query<DeliveryResolveParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryResolveEntryQuery {
        site_code,
        channel_code: params.channel_code,
        locale: params.locale,
        slug: params.slug.unwrap_or_default(),
        preview_token: params.preview_token,
    };
    match state.service.delivery_resolve_entry(&ctx, query).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "uuid": entry.uuid, "title": entry.title, "slug": entry.slug, "locale": entry.locale}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delivery_retrieve_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
    Query(params): Query<DeliveryResolveParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryRetrieveEntryQuery {
        entry_id,
        preview_token: params.preview_token,
    };
    match state.service.delivery_retrieve_entry(&ctx, query).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "uuid": entry.uuid, "title": entry.title, "slug": entry.slug, "locale": entry.locale}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delivery_resolve_page(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(site_code): Path<String>,
    Query(params): Query<DeliveryResolveParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryResolvePageQuery {
        site_code,
        channel_code: params.channel_code,
        locale: params.locale,
        path: params.path.unwrap_or_default(),
        preview_token: params.preview_token,
    };
    match state.service.delivery_resolve_page(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"id": page.id.to_string(), "title": page.title, "path": page.path, "locale": page.locale}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delivery_list_feed_items(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path((site_code, feed_code)): Path<(String, String)>,
    Query(params): Query<DeliveryFeedItemsParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryFeedItemsQuery {
        site_code,
        feed_code,
        channel_code: params.channel_code,
        locale: params.locale,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.delivery_list_feed_items(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|i| serde_json::json!({"id": i.id.to_string(), "feedId": i.feed_id.to_string(), "entryId": i.entry_id.map(|id| id.to_string()), "itemKind": i.item_kind})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

// ============ Open API Handlers ============

pub async fn open_list_entries(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryListEntriesQuery {
        site_code: "default".to_string(),
        channel_code: None,
        locale: None,
        content_type_code: None,
        term_code: None,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.delivery_list_entries(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|e| serde_json::json!({"id": e.id.to_string(), "title": e.title, "slug": e.slug, "locale": e.locale})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn open_retrieve_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(entry_id): Path<i64>,
) -> Json<serde_json::Value> {
    let query = DeliveryRetrieveEntryQuery {
        entry_id,
        preview_token: None,
    };
    match state.service.delivery_retrieve_entry(&ctx, query).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title, "slug": entry.slug}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn open_resolve_entry(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<DeliveryResolveParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryResolveEntryQuery {
        site_code: "default".to_string(),
        channel_code: params.channel_code,
        locale: params.locale,
        slug: params.slug.unwrap_or_default(),
        preview_token: None,
    };
    match state.service.delivery_resolve_entry(&ctx, query).await {
        Ok(entry) => Json(
            serde_json::json!({"ok": true, "data": {"id": entry.id.to_string(), "title": entry.title, "slug": entry.slug}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn open_resolve_page(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<DeliveryResolveParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryResolvePageQuery {
        site_code: "default".to_string(),
        channel_code: params.channel_code,
        locale: params.locale,
        path: params.path.unwrap_or_default(),
        preview_token: None,
    };
    match state.service.delivery_resolve_page(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"id": page.id.to_string(), "title": page.title, "path": page.path}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn open_list_feed_items(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(feed_code): Path<String>,
    Query(params): Query<DeliveryFeedItemsParams>,
) -> Json<serde_json::Value> {
    let query = DeliveryFeedItemsQuery {
        site_code: "default".to_string(),
        feed_code,
        channel_code: params.channel_code,
        locale: params.locale,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.delivery_list_feed_items(&ctx, query).await {
        Ok(page) => Json(
            serde_json::json!({"ok": true, "data": {"items": page.items.iter().map(|i| serde_json::json!({"id": i.id.to_string(), "feedId": i.feed_id.to_string(), "itemKind": i.item_kind})).collect::<Vec<_>>()}}),
        ),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

// ============ Favorites Handlers (App API) ============

fn favorite_json(favorite: &sdkwork_content_cms_service::domain::CmsFavorite) -> serde_json::Value {
    serde_json::json!({
        "id": favorite.id.to_string(),
        "favoriteId": favorite.uuid,
        "favoriteType": favorite.favorite_type,
        "targetType": favorite.target_type,
        "targetId": favorite.target_id.to_string(),
        "targetUuid": favorite.target_uuid,
        "targetUrl": favorite.target_url,
        "title": favorite.title,
        "summary": favorite.summary,
        "sourceDisplayName": favorite.source_display_name,
        "media": serde_json::from_str::<serde_json::Value>(&favorite.media_json)
            .unwrap_or_else(|_| serde_json::json!({})),
        "favoritedAt": favorite.favorited_at,
    })
}

pub async fn create_favorite(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Json(req): Json<CreateFavoriteRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let command = FavoriteCommand {
        favorite_type: req.favorite_type,
        target_type: req.target_type,
        target_id: req.target_id.unwrap_or(0),
        target_uuid: req.target_uuid,
        target_url: req.target_url,
        title: req.title,
        summary: req.summary,
        source_display_name: req.source_display_name,
        media_json: req
            .media
            .map(|value| value.to_string())
            .unwrap_or_else(|| "{}".to_string()),
    };
    match state.service.create_favorite(&ctx, command).await {
        Ok(favorite) => (
            StatusCode::OK,
            Json(serde_json::json!({"ok": true, "data": {"item": favorite_json(&favorite)}})),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}

pub async fn list_favorites(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Query(params): Query<FavoriteListParams>,
) -> Json<serde_json::Value> {
    let query = ListFavoritesQuery {
        favorite_type: params.favorite_type,
        search_query: params.q,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match state.service.list_favorites(&ctx, query).await {
        Ok(page) => Json(serde_json::json!({
            "ok": true,
            "data": {
                "items": page.items.iter().map(favorite_json).collect::<Vec<_>>(),
                "pageInfo": {
                    "mode": "cursor",
                    "nextCursor": page.next_cursor,
                    "hasMore": page.next_cursor.is_some(),
                },
            }
        })),
        Err(e) => Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
    }
}

pub async fn delete_favorite(
    State(state): State<AppState>,
    CmsHttpRequestContext(ctx): CmsHttpRequestContext,
    Path(favorite_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    match state.service.delete_favorite(&ctx, favorite_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"ok": true, "data": {"deleted": true}})),
        ),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"ok": false, "error": {"detail": e.to_string()}})),
        ),
    }
}
