use std::sync::Arc;
use async_trait::async_trait;
use sdkwork_content_cms_service::context::{CmsLoginScope, CmsRequestContext};
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::{CmsError, CmsResult};
use sdkwork_content_cms_service::ports::{CmsRepository, CmsIamAuthorizer, CmsEventPublisher};
use sdkwork_content_cms_service::CmsService;

// Test context factory
fn test_context(permissions: Vec<&str>) -> CmsRequestContext {
    CmsRequestContext {
        request_id: "test-req-001".to_string(),
        trace_id: Some("test-trace-001".to_string()),
        tenant_id: 100_001,
        organization_id: 0,
        user_id: 1,
        session_id: Some("session-001".to_string()),
        permissions: permissions.into_iter().map(|s| s.to_string()).collect(),
        data_scope: 2,
        login_scope: CmsLoginScope::Tenant,
    }
}

// Mock repository that returns configurable results
struct MockRepository {
    should_fail: bool,
}

impl MockRepository {
    fn new() -> Self {
        Self { should_fail: false }
    }

    fn with_failure() -> Self {
        Self { should_fail: true }
    }
}

#[async_trait]
impl CmsRepository for MockRepository {
    async fn list_sites(&self, _ctx: &CmsRequestContext, _query: ListSitesQuery) -> CmsResult<CmsSitePage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_site(&self, _ctx: &CmsRequestContext, _command: SiteCommand) -> CmsResult<CmsSite> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsSite {
            id: 1,
            uuid: "test-uuid".to_string(),
            tenant_id: 100_001,
            organization_id: 0,
            code: "test-site".to_string(),
            name: "Test Site".to_string(),
            default_locale: "en".to_string(),
            settings_json: "{}".to_string(),
            status: 1,
            version: 0,
        })
    }

    async fn retrieve_site(&self, _ctx: &CmsRequestContext, site_id: CmsId) -> CmsResult<CmsSite> {
        if self.should_fail { return Err(CmsError::not_found("site")); }
        Ok(CmsSite {
            id: site_id,
            uuid: "test-uuid".to_string(),
            tenant_id: 100_001,
            organization_id: 0,
            code: "test-site".to_string(),
            name: "Test Site".to_string(),
            default_locale: "en".to_string(),
            settings_json: "{}".to_string(),
            status: 1,
            version: 0,
        })
    }

    async fn update_site(&self, _ctx: &CmsRequestContext, site_id: CmsId, _command: SiteCommand) -> CmsResult<CmsSite> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsSite {
            id: site_id,
            uuid: "test-uuid".to_string(),
            tenant_id: 100_001,
            organization_id: 0,
            code: "updated-site".to_string(),
            name: "Updated Site".to_string(),
            default_locale: "en".to_string(),
            settings_json: "{}".to_string(),
            status: 1,
            version: 1,
        })
    }

    async fn delete_site(&self, _ctx: &CmsRequestContext, site_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(site_id), request_id: Some("test".to_string()) })
    }

    async fn list_channels(&self, _ctx: &CmsRequestContext, _query: ListBySiteQuery) -> CmsResult<CmsChannelPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_channel(&self, _ctx: &CmsRequestContext, _command: ChannelCommand) -> CmsResult<CmsChannel> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsChannel { id: 1, site_id: 1, code: "ch1".to_string(), name: "Channel 1".to_string(), channel_kind: "web".to_string(), status: 1 })
    }

    async fn update_channel(&self, _ctx: &CmsRequestContext, _channel_id: CmsId, _command: ChannelCommand) -> CmsResult<CmsChannel> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsChannel { id: _channel_id, site_id: 1, code: "ch1".to_string(), name: "Updated".to_string(), channel_kind: "web".to_string(), status: 1 })
    }

    async fn delete_channel(&self, _ctx: &CmsRequestContext, _channel_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_channel_id), request_id: Some("test".to_string()) })
    }

    async fn list_content_types(&self, _ctx: &CmsRequestContext, _query: ListBySiteQuery) -> CmsResult<CmsContentTypePage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_content_type(&self, _ctx: &CmsRequestContext, _command: ContentTypeCommand) -> CmsResult<CmsContentType> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsContentType { id: 1, site_id: 1, code: "article".to_string(), name: "Article".to_string(), content_kind: "entry".to_string(), schema_version: 1, status: 1 })
    }

    async fn retrieve_content_type(&self, _ctx: &CmsRequestContext, _content_type_id: CmsId) -> CmsResult<CmsContentType> {
        if self.should_fail { return Err(CmsError::not_found("content_type")); }
        Ok(CmsContentType { id: _content_type_id, site_id: 1, code: "article".to_string(), name: "Article".to_string(), content_kind: "entry".to_string(), schema_version: 1, status: 1 })
    }

    async fn update_content_type(&self, _ctx: &CmsRequestContext, _content_type_id: CmsId, _command: ContentTypeCommand) -> CmsResult<CmsContentType> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsContentType { id: _content_type_id, site_id: 1, code: "article".to_string(), name: "Updated".to_string(), content_kind: "entry".to_string(), schema_version: 2, status: 1 })
    }

    async fn delete_content_type(&self, _ctx: &CmsRequestContext, _content_type_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_content_type_id), request_id: Some("test".to_string()) })
    }

    async fn list_content_fields(&self, _ctx: &CmsRequestContext, _query: ListContentFieldsQuery) -> CmsResult<CmsContentFieldPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_content_field(&self, _ctx: &CmsRequestContext, _command: ContentFieldCommand) -> CmsResult<CmsContentField> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsContentField { id: 1, content_type_id: 1, code: "title".to_string(), name: "Title".to_string(), field_kind: "text".to_string(), required: true, searchable: true, filterable: false, sortable: true })
    }

    async fn update_content_field(&self, _ctx: &CmsRequestContext, _field_id: CmsId, _command: ContentFieldCommand) -> CmsResult<CmsContentField> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsContentField { id: _field_id, content_type_id: 1, code: "title".to_string(), name: "Updated".to_string(), field_kind: "text".to_string(), required: true, searchable: true, filterable: false, sortable: true })
    }

    async fn delete_content_field(&self, _ctx: &CmsRequestContext, _field_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_field_id), request_id: Some("test".to_string()) })
    }

    async fn list_taxonomies(&self, _ctx: &CmsRequestContext, _query: ListBySiteQuery) -> CmsResult<CmsTaxonomyPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_taxonomy(&self, _ctx: &CmsRequestContext, _command: TaxonomyCommand) -> CmsResult<CmsTaxonomy> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsTaxonomy { id: 1, site_id: 1, code: "category".to_string(), name: "Category".to_string(), taxonomy_kind: "category".to_string(), status: 1 })
    }

    async fn update_taxonomy(&self, _ctx: &CmsRequestContext, _taxonomy_id: CmsId, _command: TaxonomyCommand) -> CmsResult<CmsTaxonomy> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsTaxonomy { id: _taxonomy_id, site_id: 1, code: "category".to_string(), name: "Updated".to_string(), taxonomy_kind: "category".to_string(), status: 1 })
    }

    async fn delete_taxonomy(&self, _ctx: &CmsRequestContext, _taxonomy_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_taxonomy_id), request_id: Some("test".to_string()) })
    }

    async fn list_taxonomy_terms(&self, _ctx: &CmsRequestContext, _query: ListTaxonomyTermsQuery) -> CmsResult<CmsTaxonomyTermPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_taxonomy_term(&self, _ctx: &CmsRequestContext, _command: TaxonomyTermCommand) -> CmsResult<CmsTaxonomyTerm> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsTaxonomyTerm { id: 1, taxonomy_id: 1, parent_id: None, code: "tech".to_string(), slug: "tech".to_string(), name: "Technology".to_string(), path: "/tech".to_string(), status: 1 })
    }

    async fn update_taxonomy_term(&self, _ctx: &CmsRequestContext, _term_id: CmsId, _command: TaxonomyTermCommand) -> CmsResult<CmsTaxonomyTerm> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsTaxonomyTerm { id: _term_id, taxonomy_id: 1, parent_id: None, code: "tech".to_string(), slug: "tech".to_string(), name: "Updated".to_string(), path: "/tech".to_string(), status: 1 })
    }

    async fn delete_taxonomy_term(&self, _ctx: &CmsRequestContext, _term_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_term_id), request_id: Some("test".to_string()) })
    }

    async fn create_entry(&self, _ctx: &CmsRequestContext, _command: EntryCommand) -> CmsResult<CmsEntry> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsEntry {
            id: 1, uuid: "entry-uuid".to_string(), site_id: 1, content_type_id: 1, channel_id: Some(1),
            locale: "en".to_string(), title: "Test Entry".to_string(), slug: "test-entry".to_string(),
            summary: None, entry_status: CmsEntryStatus::Draft, publication_status: CmsPublicationStatus::Unpublished,
            published_at: None, version: 0,
        })
    }

    async fn update_entry(&self, _ctx: &CmsRequestContext, _entry_id: CmsId, _command: EntryCommand) -> CmsResult<CmsEntry> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsEntry {
            id: _entry_id, uuid: "entry-uuid".to_string(), site_id: 1, content_type_id: 1, channel_id: Some(1),
            locale: "en".to_string(), title: "Updated Entry".to_string(), slug: "updated-entry".to_string(),
            summary: None, entry_status: CmsEntryStatus::Draft, publication_status: CmsPublicationStatus::Unpublished,
            published_at: None, version: 1,
        })
    }

    async fn delete_entry(&self, _ctx: &CmsRequestContext, _entry_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_entry_id), request_id: Some("test".to_string()) })
    }

    async fn retrieve_entry(&self, _ctx: &CmsRequestContext, _entry_id: CmsId) -> CmsResult<CmsEntry> {
        if self.should_fail { return Err(CmsError::not_found("entry")); }
        Ok(CmsEntry {
            id: _entry_id, uuid: "entry-uuid".to_string(), site_id: 1, content_type_id: 1, channel_id: Some(1),
            locale: "en".to_string(), title: "Test Entry".to_string(), slug: "test-entry".to_string(),
            summary: None, entry_status: CmsEntryStatus::Draft, publication_status: CmsPublicationStatus::Unpublished,
            published_at: None, version: 0,
        })
    }

    async fn list_entries(&self, _ctx: &CmsRequestContext, _query: ListEntriesQuery) -> CmsResult<CmsEntryPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn replace_entry_body(&self, _ctx: &CmsRequestContext, _command: EntryBodyCommand) -> CmsResult<CmsEntry> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsEntry {
            id: _command.entry_id, uuid: "entry-uuid".to_string(), site_id: 1, content_type_id: 1, channel_id: Some(1),
            locale: "en".to_string(), title: "Test Entry".to_string(), slug: "test-entry".to_string(),
            summary: None, entry_status: CmsEntryStatus::Draft, publication_status: CmsPublicationStatus::Unpublished,
            published_at: None, version: 1,
        })
    }

    async fn replace_entry_fields(&self, _ctx: &CmsRequestContext, _command: EntryFieldsCommand) -> CmsResult<CmsEntry> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsEntry {
            id: _command.entry_id, uuid: "entry-uuid".to_string(), site_id: 1, content_type_id: 1, channel_id: Some(1),
            locale: "en".to_string(), title: "Test Entry".to_string(), slug: "test-entry".to_string(),
            summary: None, entry_status: CmsEntryStatus::Draft, publication_status: CmsPublicationStatus::Unpublished,
            published_at: None, version: 1,
        })
    }

    async fn list_entry_media(&self, _ctx: &CmsRequestContext, _query: ListEntryMediaQuery) -> CmsResult<CmsMediaRefPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn attach_entry_media(&self, _ctx: &CmsRequestContext, _command: EntryMediaCommand) -> CmsResult<CmsMediaRef> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsMediaRef { id: 1, role: "featured".to_string(), drive_space_id: None, drive_node_id: None, drive_uri: None, media_resource_id: None, media_snapshot_json: "{}".to_string() })
    }

    async fn delete_entry_media(&self, _ctx: &CmsRequestContext, _media_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_media_id), request_id: Some("test".to_string()) })
    }

    async fn replace_entry_terms(&self, _ctx: &CmsRequestContext, _command: ReplaceEntryTermsCommand) -> CmsResult<CmsEntry> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsEntry {
            id: _command.entry_id, uuid: "entry-uuid".to_string(), site_id: 1, content_type_id: 1, channel_id: Some(1),
            locale: "en".to_string(), title: "Test Entry".to_string(), slug: "test-entry".to_string(),
            summary: None, entry_status: CmsEntryStatus::Draft, publication_status: CmsPublicationStatus::Unpublished,
            published_at: None, version: 1,
        })
    }

    async fn list_entry_versions(&self, _ctx: &CmsRequestContext, _query: ListEntryVersionsQuery) -> CmsResult<CmsEntryVersionPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_publish_snapshot(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPublishSnapshot { id: 1, owner_type: "entry".to_string(), owner_id: 1, snapshot_payload_json: "{}".to_string(), status: 1, published_at: "2026-01-01T00:00:00Z".to_string() })
    }

    async fn publish_entry(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPublishSnapshot { id: 1, owner_type: "entry".to_string(), owner_id: _command.owner_id, snapshot_payload_json: "{}".to_string(), status: 1, published_at: "2026-01-01T00:00:00Z".to_string() })
    }

    async fn unpublish_entry(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPublishSnapshot { id: 2, owner_type: "entry".to_string(), owner_id: _command.owner_id, snapshot_payload_json: "{}".to_string(), status: 2, published_at: "2026-01-01T00:00:00Z".to_string() })
    }

    async fn rollback_entry(&self, _ctx: &CmsRequestContext, _command: RollbackCommand) -> CmsResult<CmsPublishSnapshot> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPublishSnapshot { id: 3, owner_type: "entry".to_string(), owner_id: _command.owner_id, snapshot_payload_json: "{}".to_string(), status: 1, published_at: "2026-01-01T00:00:00Z".to_string() })
    }

    async fn schedule_entry(&self, _ctx: &CmsRequestContext, _command: ScheduleCommand) -> CmsResult<CmsEntry> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsEntry {
            id: _command.entry_id, uuid: "entry-uuid".to_string(), site_id: 1, content_type_id: 1, channel_id: Some(1),
            locale: "en".to_string(), title: "Test Entry".to_string(), slug: "test-entry".to_string(),
            summary: None, entry_status: CmsEntryStatus::Draft, publication_status: CmsPublicationStatus::Scheduled,
            published_at: None, version: 1,
        })
    }

    async fn list_pages(&self, _ctx: &CmsRequestContext, _query: ListPagesQuery) -> CmsResult<CmsPagePage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_page(&self, _ctx: &CmsRequestContext, _command: PageCommand) -> CmsResult<CmsPageModel> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPageModel { id: 1, site_id: 1, channel_id: Some(1), locale: "en".to_string(), path: "/test".to_string(), title: "Test Page".to_string(), publication_status: CmsPublicationStatus::Unpublished, version: 0 })
    }

    async fn retrieve_page(&self, _ctx: &CmsRequestContext, _page_id: CmsId) -> CmsResult<CmsPageModel> {
        if self.should_fail { return Err(CmsError::not_found("page")); }
        Ok(CmsPageModel { id: _page_id, site_id: 1, channel_id: Some(1), locale: "en".to_string(), path: "/test".to_string(), title: "Test Page".to_string(), publication_status: CmsPublicationStatus::Unpublished, version: 0 })
    }

    async fn update_page(&self, _ctx: &CmsRequestContext, _page_id: CmsId, _command: PageCommand) -> CmsResult<CmsPageModel> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPageModel { id: _page_id, site_id: 1, channel_id: Some(1), locale: "en".to_string(), path: "/test".to_string(), title: "Updated Page".to_string(), publication_status: CmsPublicationStatus::Unpublished, version: 1 })
    }

    async fn delete_page(&self, _ctx: &CmsRequestContext, _page_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_page_id), request_id: Some("test".to_string()) })
    }

    async fn replace_page_blocks(&self, _ctx: &CmsRequestContext, _command: PageBlocksCommand) -> CmsResult<CmsPageModel> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPageModel { id: _command.page_id, site_id: 1, channel_id: Some(1), locale: "en".to_string(), path: "/test".to_string(), title: "Test Page".to_string(), publication_status: CmsPublicationStatus::Unpublished, version: 1 })
    }

    async fn publish_page(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPublishSnapshot { id: 1, owner_type: "page".to_string(), owner_id: _command.owner_id, snapshot_payload_json: "{}".to_string(), status: 1, published_at: "2026-01-01T00:00:00Z".to_string() })
    }

    async fn list_feeds(&self, _ctx: &CmsRequestContext, _query: ListFeedsQuery) -> CmsResult<CmsFeedPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_feed(&self, _ctx: &CmsRequestContext, _command: FeedCommand) -> CmsResult<CmsFeed> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsFeed { id: 1, site_id: 1, channel_id: Some(1), code: "latest".to_string(), name: "Latest".to_string(), feed_kind: "hybrid".to_string(), locale: "en".to_string(), status: 1, version: 0 })
    }

    async fn retrieve_feed(&self, _ctx: &CmsRequestContext, _feed_id: CmsId) -> CmsResult<CmsFeed> {
        if self.should_fail { return Err(CmsError::not_found("feed")); }
        Ok(CmsFeed { id: _feed_id, site_id: 1, channel_id: Some(1), code: "latest".to_string(), name: "Latest".to_string(), feed_kind: "hybrid".to_string(), locale: "en".to_string(), status: 1, version: 0 })
    }

    async fn update_feed(&self, _ctx: &CmsRequestContext, _feed_id: CmsId, _command: FeedCommand) -> CmsResult<CmsFeed> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsFeed { id: _feed_id, site_id: 1, channel_id: Some(1), code: "latest".to_string(), name: "Updated".to_string(), feed_kind: "hybrid".to_string(), locale: "en".to_string(), status: 1, version: 1 })
    }

    async fn delete_feed(&self, _ctx: &CmsRequestContext, _feed_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_feed_id), request_id: Some("test".to_string()) })
    }

    async fn list_feed_rules(&self, _ctx: &CmsRequestContext, _query: ListFeedRulesQuery) -> CmsResult<CmsFeedRulePage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn create_feed_rule(&self, _ctx: &CmsRequestContext, _command: FeedRuleCommand) -> CmsResult<CmsFeedRule> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsFeedRule { id: 1, feed_id: 1, rule_kind: "latest".to_string(), condition_json: "{}".to_string(), sort_json: "{}".to_string(), enabled: true })
    }

    async fn update_feed_rule(&self, _ctx: &CmsRequestContext, _rule_id: CmsId, _command: FeedRuleCommand) -> CmsResult<CmsFeedRule> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsFeedRule { id: _rule_id, feed_id: 1, rule_kind: "latest".to_string(), condition_json: "{}".to_string(), sort_json: "{}".to_string(), enabled: true })
    }

    async fn delete_feed_rule(&self, _ctx: &CmsRequestContext, _rule_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_rule_id), request_id: Some("test".to_string()) })
    }

    async fn list_feed_items(&self, _ctx: &CmsRequestContext, _query: ListFeedItemsQuery) -> CmsResult<CmsFeedItemPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn upsert_feed_items(&self, _ctx: &CmsRequestContext, _command: FeedItemsCommand) -> CmsResult<CmsFeedItemPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn delete_feed_item(&self, _ctx: &CmsRequestContext, _item_id: CmsId) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_item_id), request_id: Some("test".to_string()) })
    }

    async fn retrieve_feed_snapshot(&self, _ctx: &CmsRequestContext, _snapshot_id: CmsId) -> CmsResult<CmsFeedSnapshot> {
        if self.should_fail { return Err(CmsError::not_found("feed_snapshot")); }
        Ok(CmsFeedSnapshot { id: _snapshot_id, feed_id: 1, publish_snapshot_id: Some(1), snapshot_version: 1, item_count: 0, items_json: "[]".to_string(), status: 1, published_at: "2026-01-01T00:00:00Z".to_string() })
    }

    async fn publish_feed(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPublishSnapshot { id: 1, owner_type: "feed".to_string(), owner_id: _command.owner_id, snapshot_payload_json: "{}".to_string(), status: 1, published_at: "2026-01-01T00:00:00Z".to_string() })
    }

    async fn create_favorite(&self, _ctx: &CmsRequestContext, _command: FavoriteCommand) -> CmsResult<CmsFavorite> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsFavorite {
            id: 1, uuid: "fav-1".to_string(), tenant_id: _ctx.tenant_id, organization_id: _ctx.organization_id,
            user_id: _ctx.user_id, favorite_type: _command.favorite_type, target_type: _command.target_type,
            target_id: _command.target_id, target_uuid: _command.target_uuid, target_url: _command.target_url,
            title: _command.title, summary: _command.summary, source_display_name: _command.source_display_name,
            media_json: _command.media_json, favorited_at: "2026-01-01T00:00:00Z".to_string(), version: 0,
        })
    }

    async fn list_favorites(&self, _ctx: &CmsRequestContext, _query: ListFavoritesQuery) -> CmsResult<CmsFavoritePage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn delete_favorite(&self, _ctx: &CmsRequestContext, _favorite_uuid: String) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: None, request_id: Some("test".to_string()) })
    }

    async fn list_audit_logs(&self, _ctx: &CmsRequestContext, _query: ListAuditLogsQuery) -> CmsResult<CmsAuditLogPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn list_outbox_events(&self, _ctx: &CmsRequestContext, _query: ListOutboxEventsQuery) -> CmsResult<CmsOutboxEventPage> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CmsPage { items: vec![], next_cursor: None })
    }

    async fn retry_outbox_event(&self, _ctx: &CmsRequestContext, _command: RetryOutboxEventCommand) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(_command.event_id), request_id: Some("test".to_string()) })
    }

    async fn create_outbox_event(&self, _ctx: &CmsRequestContext, _event: CmsOutboxEventDraft) -> CmsResult<CommandResult> {
        if self.should_fail { return Err(CmsError::internal("db error")); }
        Ok(CommandResult { ok: true, resource_id: Some(1), request_id: Some("test".to_string()) })
    }
}

// Mock authorizer
struct MockAuthorizer {
    should_deny: bool,
}

impl MockAuthorizer {
    fn allow() -> Self {
        Self { should_deny: false }
    }

    fn deny() -> Self {
        Self { should_deny: true }
    }
}

#[async_trait]
impl CmsIamAuthorizer for MockAuthorizer {
    async fn require_permission(&self, _ctx: &CmsRequestContext, _permission: &'static str) -> CmsResult<()> {
        if self.should_deny {
            Err(CmsError::permission_denied(_permission))
        } else {
            Ok(())
        }
    }
}

// Mock event publisher
struct MockEventPublisher;

#[async_trait]
impl CmsEventPublisher for MockEventPublisher {
    async fn enqueue(&self, _ctx: &CmsRequestContext, _event: CmsOutboxEventDraft) -> CmsResult<()> {
        Ok(())
    }
}

// Helper to create service
fn create_service(should_fail_repo: bool, should_deny_auth: bool) -> CmsService {
    let repository = Arc::new(MockRepository::with_failure());
    let authorizer = Arc::new(if should_deny_auth { MockAuthorizer::deny() } else { MockAuthorizer::allow() });
    let event_publisher = Arc::new(MockEventPublisher);
    CmsService::new(
        if should_fail_repo { repository } else { Arc::new(MockRepository::new()) },
        authorizer,
        event_publisher,
    )
}

// ===== Permission Tests =====

#[tokio::test]
async fn permission_denied_for_site_read_without_permission() {
    let service = create_service(false, true);
    let ctx = test_context(vec![]);
    let result = service.list_sites(&ctx, ListSitesQuery { cursor: None, limit: 10 }).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        CmsError::PermissionDenied(_) => {},
        other => panic!("Expected PermissionDenied, got: {:?}", other),
    }
}

#[tokio::test]
async fn permission_denied_for_site_manage_without_permission() {
    let service = create_service(false, true);
    let ctx = test_context(vec![]);
    let result = service.create_site(&ctx, SiteCommand {
        code: Some("test".to_string()),
        name: Some("Test".to_string()),
        description: None,
        default_locale: Some("en".to_string()),
        settings_json: None,
        version: None,
    }).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        CmsError::PermissionDenied(_) => {},
        other => panic!("Expected PermissionDenied, got: {:?}", other),
    }
}

#[tokio::test]
async fn permission_denied_for_entry_create_without_permission() {
    let service = create_service(false, true);
    let ctx = test_context(vec![]);
    let result = service.create_entry(&ctx, EntryCommand {
        site_id: 1,
        content_type_id: 1,
        channel_id: Some(1),
        locale: "en".to_string(),
        title: "Test".to_string(),
        slug: "test".to_string(),
        summary: None,
        seo_json: None,
        version: None,
    }).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        CmsError::PermissionDenied(_) => {},
        other => panic!("Expected PermissionDenied, got: {:?}", other),
    }
}

#[tokio::test]
async fn permission_denied_for_entry_publish_without_permission() {
    let service = create_service(false, true);
    let ctx = test_context(vec![]);
    let result = service.publish_entry(&ctx, PublishCommand {
        owner_type: "entry".to_string(),
        owner_id: 1,
        channel_id: None,
        locale: None,
        note: None,
        version: None,
    }).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        CmsError::PermissionDenied(_) => {},
        other => panic!("Expected PermissionDenied, got: {:?}", other),
    }
}

// ===== Site CRUD Tests =====

#[tokio::test]
async fn list_sites_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.site.read"]);
    let result = service.list_sites(&ctx, ListSitesQuery { cursor: None, limit: 10 }).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_site_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.site.manage"]);
    let result = service.create_site(&ctx, SiteCommand {
        code: Some("test".to_string()),
        name: Some("Test Site".to_string()),
        description: None,
        default_locale: Some("en".to_string()),
        settings_json: None,
        version: None,
    }).await;
    assert!(result.is_ok());
    let site = result.unwrap();
    assert_eq!(site.code, "test-site");
}

#[tokio::test]
async fn retrieve_site_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.site.read"]);
    let result = service.retrieve_site(&ctx, 1).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, 1);
}

#[tokio::test]
async fn update_site_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.site.manage"]);
    let result = service.update_site(&ctx, 1, SiteCommand {
        code: Some("updated".to_string()),
        name: Some("Updated Site".to_string()),
        description: None,
        default_locale: Some("en".to_string()),
        settings_json: None,
        version: Some(0),
    }).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().code, "updated-site");
}

#[tokio::test]
async fn delete_site_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.site.manage"]);
    let result = service.delete_site(&ctx, 1).await;
    assert!(result.is_ok());
    assert!(result.unwrap().ok);
}

// ===== Entry Lifecycle Tests =====

#[tokio::test]
async fn create_entry_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.entry.create"]);
    let result = service.create_entry(&ctx, EntryCommand {
        site_id: 1,
        content_type_id: 1,
        channel_id: Some(1),
        locale: "en".to_string(),
        title: "Test Entry".to_string(),
        slug: "test-entry".to_string(),
        summary: None,
        seo_json: None,
        version: None,
    }).await;
    assert!(result.is_ok());
    let entry = result.unwrap();
    assert_eq!(entry.title, "Test Entry");
    assert_eq!(entry.entry_status, CmsEntryStatus::Draft);
}

#[tokio::test]
async fn update_entry_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.entry.update"]);
    let result = service.update_entry(&ctx, 1, EntryCommand {
        site_id: 1,
        content_type_id: 1,
        channel_id: Some(1),
        locale: "en".to_string(),
        title: "Updated Entry".to_string(),
        slug: "updated-entry".to_string(),
        summary: None,
        seo_json: None,
        version: Some(0),
    }).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().title, "Updated Entry");
}

#[tokio::test]
async fn delete_entry_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.entry.delete"]);
    let result = service.delete_entry(&ctx, 1).await;
    assert!(result.is_ok());
    assert!(result.unwrap().ok);
}

#[tokio::test]
async fn publish_entry_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.entry.publish"]);
    let result = service.publish_entry(&ctx, PublishCommand {
        owner_type: "entry".to_string(),
        owner_id: 1,
        channel_id: Some(1),
        locale: Some("en".to_string()),
        note: None,
        version: None,
    }).await;
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    assert_eq!(snapshot.owner_type, "entry");
    assert_eq!(snapshot.status, 1);
}

#[tokio::test]
async fn unpublish_entry_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.entry.publish"]);
    let result = service.unpublish_entry(&ctx, PublishCommand {
        owner_type: "entry".to_string(),
        owner_id: 1,
        channel_id: None,
        locale: None,
        note: None,
        version: None,
    }).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, 2);
}

#[tokio::test]
async fn rollback_entry_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.entry.rollback"]);
    let result = service.rollback_entry(&ctx, RollbackCommand {
        owner_type: "entry".to_string(),
        owner_id: 1,
        target_version_id: 1,
        note: None,
        version: None,
    }).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn schedule_entry_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.entry.publish"]);
    let result = service.schedule_entry(&ctx, ScheduleCommand {
        entry_id: 1,
        scheduled_publish_at: Some("2026-06-01T00:00:00Z".to_string()),
        scheduled_unpublish_at: None,
        version: None,
    }).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().publication_status, CmsPublicationStatus::Scheduled);
}

// ===== Content Type Tests =====

#[tokio::test]
async fn create_content_type_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.content_type.manage"]);
    let result = service.create_content_type(&ctx, ContentTypeCommand {
        site_id: 1,
        code: Some("article".to_string()),
        name: Some("Article".to_string()),
        content_kind: Some("entry".to_string()),
        settings_json: None,
        version: None,
    }).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().code, "article");
}

#[tokio::test]
async fn list_content_types_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.content_type.read"]);
    let result = service.list_content_types(&ctx, ListBySiteQuery { site_id: 1, cursor: None, limit: 10 }).await;
    assert!(result.is_ok());
}

// ===== Feed Tests =====

#[tokio::test]
async fn create_feed_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.feed.manage"]);
    let result = service.create_feed(&ctx, FeedCommand {
        site_id: 1,
        channel_id: Some(1),
        code: Some("latest".to_string()),
        name: Some("Latest".to_string()),
        feed_kind: Some("hybrid".to_string()),
        locale: Some("en".to_string()),
        rule_config_json: None,
        version: None,
    }).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().code, "latest");
}

#[tokio::test]
async fn publish_feed_with_permission() {
    let service = create_service(false, false);
    let ctx = test_context(vec!["cms.feed.publish"]);
    let result = service.publish_feed(&ctx, PublishCommand {
        owner_type: "feed".to_string(),
        owner_id: 1,
        channel_id: None,
        locale: None,
        note: None,
        version: None,
    }).await;
    assert!(result.is_ok());
}

// ===== Repository Error Tests =====

#[tokio::test]
async fn repository_error_propagated() {
    let service = create_service(true, false);
    let ctx = test_context(vec!["cms.site.read"]);
    let result = service.list_sites(&ctx, ListSitesQuery { cursor: None, limit: 10 }).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        CmsError::Internal(_) => {},
        other => panic!("Expected Internal error, got: {:?}", other),
    }
}

#[tokio::test]
async fn not_found_error_propagated() {
    let service = create_service(true, false);
    let ctx = test_context(vec!["cms.site.read"]);
    let result = service.retrieve_site(&ctx, 999).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        CmsError::NotFound(_) => {},
        other => panic!("Expected NotFound, got: {:?}", other),
    }
}

// ===== Favorites Tests =====

fn favorite_command() -> FavoriteCommand {
    FavoriteCommand {
        favorite_type: "chat".to_string(),
        target_type: "im_message".to_string(),
        target_id: 42,
        target_uuid: None,
        target_url: None,
        title: "Hello".to_string(),
        summary: "preview".to_string(),
        source_display_name: "张三".to_string(),
        media_json: "{}".to_string(),
    }
}

#[tokio::test]
async fn create_favorite_requires_authenticated_user() {
    let service = create_service(false, false);
    let mut ctx = test_context(vec![]);
    ctx.user_id = 0;
    let result = service.create_favorite(&ctx, favorite_command()).await;
    assert!(matches!(result, Err(CmsError::PermissionDenied(_))));
}

#[tokio::test]
async fn create_favorite_rejects_unknown_favorite_type() {
    let service = create_service(false, false);
    let ctx = test_context(vec![]);
    let mut command = favorite_command();
    command.favorite_type = "video".to_string();
    let result = service.create_favorite(&ctx, command).await;
    assert!(matches!(result, Err(CmsError::Validation(_))));
}

#[tokio::test]
async fn create_favorite_rejects_missing_target_id() {
    let service = create_service(false, false);
    let ctx = test_context(vec![]);
    let mut command = favorite_command();
    command.target_id = 0;
    let result = service.create_favorite(&ctx, command).await;
    assert!(matches!(result, Err(CmsError::Validation(_))));
}

#[tokio::test]
async fn create_favorite_succeeds_for_authenticated_user() {
    let service = create_service(false, false);
    let ctx = test_context(vec![]);
    let result = service.create_favorite(&ctx, favorite_command()).await;
    assert!(result.is_ok());
    let favorite = result.unwrap();
    assert_eq!(favorite.target_type, "im_message");
    assert_eq!(favorite.user_id, ctx.user_id);
}

#[tokio::test]
async fn list_favorites_requires_authenticated_user() {
    let service = create_service(false, false);
    let mut ctx = test_context(vec![]);
    ctx.user_id = 0;
    let result = service
        .list_favorites(&ctx, ListFavoritesQuery { favorite_type: None, search_query: None, cursor: None, limit: 20 })
        .await;
    assert!(matches!(result, Err(CmsError::PermissionDenied(_))));
}

#[tokio::test]
async fn delete_favorite_requires_uuid() {
    let service = create_service(false, false);
    let ctx = test_context(vec![]);
    let result = service.delete_favorite(&ctx, "".to_string()).await;
    assert!(matches!(result, Err(CmsError::Validation(_))));
}
