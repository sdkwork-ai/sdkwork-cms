use super::value_objects::{CmsId, CmsInstant, CmsJson};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CmsEntryStatus {
    Draft,
    Reviewing,
    Approved,
    Published,
    Archived,
    Deleted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CmsPublicationStatus {
    Unpublished,
    Scheduled,
    Published,
    UnpublishedAfterPublish,
    RolledBack,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsSite {
    pub id: CmsId,
    pub uuid: String,
    pub tenant_id: CmsId,
    pub organization_id: CmsId,
    pub code: String,
    pub name: String,
    pub default_locale: String,
    pub settings_json: CmsJson,
    pub status: i32,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsChannel {
    pub id: CmsId,
    pub site_id: CmsId,
    pub code: String,
    pub name: String,
    pub channel_kind: String,
    pub status: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsContentType {
    pub id: CmsId,
    pub site_id: CmsId,
    pub code: String,
    pub name: String,
    pub content_kind: String,
    pub schema_version: i64,
    pub status: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsContentField {
    pub id: CmsId,
    pub content_type_id: CmsId,
    pub code: String,
    pub name: String,
    pub field_kind: String,
    pub required: bool,
    pub searchable: bool,
    pub filterable: bool,
    pub sortable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsTaxonomy {
    pub id: CmsId,
    pub site_id: CmsId,
    pub code: String,
    pub name: String,
    pub taxonomy_kind: String,
    pub status: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsTaxonomyTerm {
    pub id: CmsId,
    pub taxonomy_id: CmsId,
    pub parent_id: Option<CmsId>,
    pub code: String,
    pub slug: String,
    pub name: String,
    pub path: String,
    pub status: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsEntry {
    pub id: CmsId,
    pub uuid: String,
    pub site_id: CmsId,
    pub content_type_id: CmsId,
    pub channel_id: Option<CmsId>,
    pub locale: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub entry_status: CmsEntryStatus,
    pub publication_status: CmsPublicationStatus,
    pub published_at: Option<CmsInstant>,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsEntryBody {
    pub entry_id: CmsId,
    pub locale: String,
    pub body_format: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub block_tree_json: CmsJson,
    pub plain_text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsEntryVersion {
    pub id: CmsId,
    pub entry_id: CmsId,
    pub version_no: i64,
    pub version_kind: String,
    pub checksum: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsMediaRef {
    pub id: CmsId,
    pub role: String,
    pub drive_space_id: Option<String>,
    pub drive_node_id: Option<String>,
    pub drive_uri: Option<String>,
    pub media_resource_id: Option<String>,
    pub media_snapshot_json: CmsJson,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsPageModel {
    pub id: CmsId,
    pub site_id: CmsId,
    pub channel_id: Option<CmsId>,
    pub locale: String,
    pub path: String,
    pub title: String,
    pub publication_status: CmsPublicationStatus,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsPageBlock {
    pub id: CmsId,
    pub page_id: CmsId,
    pub slot_code: String,
    pub block_code: String,
    pub block_kind: String,
    pub ref_type: Option<String>,
    pub ref_id: Option<CmsId>,
    pub config_json: CmsJson,
    pub sort_order: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsFeed {
    pub id: CmsId,
    pub site_id: CmsId,
    pub channel_id: Option<CmsId>,
    pub code: String,
    pub name: String,
    pub feed_kind: String,
    pub locale: String,
    pub status: i32,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsFeedRule {
    pub id: CmsId,
    pub feed_id: CmsId,
    pub rule_kind: String,
    pub condition_json: CmsJson,
    pub sort_json: CmsJson,
    pub enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsFeedItem {
    pub id: CmsId,
    pub feed_id: CmsId,
    pub entry_id: Option<CmsId>,
    pub page_id: Option<CmsId>,
    pub external_url: Option<String>,
    pub item_kind: String,
    pub pinned: bool,
    pub sort_order: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsPublishSnapshot {
    pub id: CmsId,
    pub owner_type: String,
    pub owner_id: CmsId,
    pub snapshot_payload_json: CmsJson,
    pub status: i32,
    pub published_at: CmsInstant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsFeedSnapshot {
    pub id: CmsId,
    pub feed_id: CmsId,
    pub publish_snapshot_id: Option<CmsId>,
    pub snapshot_version: i64,
    pub item_count: i32,
    pub items_json: CmsJson,
    pub status: i32,
    pub published_at: CmsInstant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsFavorite {
    pub id: CmsId,
    pub uuid: String,
    pub tenant_id: CmsId,
    pub organization_id: CmsId,
    pub user_id: CmsId,
    pub favorite_type: String,
    pub target_type: String,
    pub target_id: CmsId,
    pub target_uuid: Option<String>,
    pub target_url: Option<String>,
    pub title: String,
    pub summary: String,
    pub source_display_name: String,
    pub media_json: CmsJson,
    pub favorited_at: CmsInstant,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsAuditLog {
    pub id: CmsId,
    pub site_id: Option<CmsId>,
    pub actor_user_id: CmsId,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<CmsId>,
    pub before_json: CmsJson,
    pub after_json: CmsJson,
    pub created_at: CmsInstant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsOutboxEvent {
    pub id: CmsId,
    pub aggregate_type: String,
    pub aggregate_id: CmsId,
    pub event_type: String,
    pub payload_json: CmsJson,
    pub status: i32,
    pub attempt_count: i32,
    pub next_attempt_at: Option<CmsInstant>,
    pub created_at: CmsInstant,
}
