use super::value_objects::{CmsId, CmsInstant, CmsJson};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListSitesQuery {
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListBySiteQuery {
    pub site_id: CmsId,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListContentFieldsQuery {
    pub content_type_id: CmsId,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListTaxonomyTermsQuery {
    pub taxonomy_id: CmsId,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListEntriesQuery {
    pub site_id: Option<CmsId>,
    pub content_type_id: Option<CmsId>,
    pub channel_id: Option<CmsId>,
    pub locale: Option<String>,
    pub entry_status: Option<i32>,
    pub publication_status: Option<i32>,
    pub author_user_id: Option<CmsId>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListPagesQuery {
    pub site_id: Option<CmsId>,
    pub channel_id: Option<CmsId>,
    pub locale: Option<String>,
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListFeedsQuery {
    pub site_id: Option<CmsId>,
    pub channel_id: Option<CmsId>,
    pub locale: Option<String>,
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListFeedRulesQuery {
    pub feed_id: CmsId,
    pub enabled: Option<bool>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListFeedItemsQuery {
    pub feed_id: CmsId,
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListAuditLogsQuery {
    pub site_id: Option<CmsId>,
    pub resource_type: Option<String>,
    pub resource_id: Option<CmsId>,
    pub actor_user_id: Option<CmsId>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListOutboxEventsQuery {
    pub aggregate_type: Option<String>,
    pub aggregate_id: Option<CmsId>,
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteCommand {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_locale: Option<String>,
    pub settings_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChannelCommand {
    pub site_id: CmsId,
    pub code: Option<String>,
    pub name: Option<String>,
    pub channel_kind: Option<String>,
    pub delivery_config_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentTypeCommand {
    pub site_id: CmsId,
    pub code: Option<String>,
    pub name: Option<String>,
    pub content_kind: Option<String>,
    pub settings_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentFieldCommand {
    pub content_type_id: CmsId,
    pub code: Option<String>,
    pub name: Option<String>,
    pub field_kind: Option<String>,
    pub validation_json: Option<CmsJson>,
    pub options_json: Option<CmsJson>,
    pub ui_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaxonomyCommand {
    pub site_id: CmsId,
    pub code: Option<String>,
    pub name: Option<String>,
    pub taxonomy_kind: Option<String>,
    pub settings_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaxonomyTermCommand {
    pub taxonomy_id: CmsId,
    pub parent_id: Option<CmsId>,
    pub code: Option<String>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryCommand {
    pub site_id: CmsId,
    pub content_type_id: CmsId,
    pub channel_id: Option<CmsId>,
    pub locale: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub seo_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryBodyCommand {
    pub entry_id: CmsId,
    pub locale: String,
    pub body_format: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub block_tree_json: CmsJson,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryFieldsCommand {
    pub entry_id: CmsId,
    pub locale: String,
    pub fields_json: CmsJson,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryMediaCommand {
    pub entry_id: CmsId,
    pub field_id: Option<CmsId>,
    pub media_role: String,
    pub drive_space_id: Option<String>,
    pub drive_node_id: Option<String>,
    pub drive_uri: Option<String>,
    pub media_resource_id: Option<String>,
    pub media_snapshot_json: CmsJson,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListEntryMediaQuery {
    pub entry_id: CmsId,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaceEntryTermsCommand {
    pub entry_id: CmsId,
    pub term_ids: Vec<CmsId>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListEntryVersionsQuery {
    pub entry_id: CmsId,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishCommand {
    pub owner_type: String,
    pub owner_id: CmsId,
    pub channel_id: Option<CmsId>,
    pub locale: Option<String>,
    pub note: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RollbackCommand {
    pub owner_type: String,
    pub owner_id: CmsId,
    pub target_version_id: CmsId,
    pub note: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduleCommand {
    pub entry_id: CmsId,
    pub scheduled_publish_at: Option<CmsInstant>,
    pub scheduled_unpublish_at: Option<CmsInstant>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageCommand {
    pub site_id: CmsId,
    pub channel_id: Option<CmsId>,
    pub locale: String,
    pub path: String,
    pub slug: String,
    pub title: String,
    pub seo_json: Option<CmsJson>,
    pub settings_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageBlocksCommand {
    pub page_id: CmsId,
    pub blocks_json: CmsJson,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedCommand {
    pub site_id: CmsId,
    pub channel_id: Option<CmsId>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub feed_kind: Option<String>,
    pub locale: Option<String>,
    pub rule_config_json: Option<CmsJson>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedRuleCommand {
    pub feed_id: CmsId,
    pub rule_kind: String,
    pub condition_json: CmsJson,
    pub sort_json: CmsJson,
    pub limit_count: u32,
    pub enabled: bool,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedItemsCommand {
    pub feed_id: CmsId,
    pub items_json: CmsJson,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FavoriteCommand {
    pub favorite_type: String,
    pub target_type: String,
    pub target_id: CmsId,
    pub target_uuid: Option<String>,
    pub target_url: Option<String>,
    pub title: String,
    pub summary: String,
    pub source_display_name: String,
    pub media_json: CmsJson,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListFavoritesQuery {
    pub favorite_type: Option<String>,
    pub search_query: Option<String>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetryOutboxEventCommand {
    pub event_id: CmsId,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryBootstrapQuery {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryResolveEntryQuery {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub slug: String,
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryListEntriesQuery {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub content_type_code: Option<String>,
    pub term_code: Option<String>,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryRetrieveEntryQuery {
    pub entry_id: CmsId,
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryResolvePageQuery {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub path: String,
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryFeedItemsQuery {
    pub site_code: String,
    pub feed_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub cursor: Option<String>,
    pub limit: u32,
}
