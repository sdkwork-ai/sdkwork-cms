use std::sync::Arc;
use async_trait::async_trait;
use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;
use sdkwork_content_cms_service::ports::{CmsRepository, CmsIamAuthorizer, CmsEventPublisher};
use sdkwork_content_cms_service::CmsService;

struct MockRepository;

#[async_trait]
impl CmsRepository for MockRepository {
    async fn list_sites(&self, _ctx: &CmsRequestContext, _query: ListSitesQuery) -> CmsResult<CmsSitePage> { unimplemented!() }
    async fn create_site(&self, _ctx: &CmsRequestContext, _command: SiteCommand) -> CmsResult<CmsSite> { unimplemented!() }
    async fn retrieve_site(&self, _ctx: &CmsRequestContext, _site_id: CmsId) -> CmsResult<CmsSite> { unimplemented!() }
    async fn update_site(&self, _ctx: &CmsRequestContext, _site_id: CmsId, _command: SiteCommand) -> CmsResult<CmsSite> { unimplemented!() }
    async fn delete_site(&self, _ctx: &CmsRequestContext, _site_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_channels(&self, _ctx: &CmsRequestContext, _query: ListBySiteQuery) -> CmsResult<CmsChannelPage> { unimplemented!() }
    async fn create_channel(&self, _ctx: &CmsRequestContext, _command: ChannelCommand) -> CmsResult<CmsChannel> { unimplemented!() }
    async fn update_channel(&self, _ctx: &CmsRequestContext, _channel_id: CmsId, _command: ChannelCommand) -> CmsResult<CmsChannel> { unimplemented!() }
    async fn delete_channel(&self, _ctx: &CmsRequestContext, _channel_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_content_types(&self, _ctx: &CmsRequestContext, _query: ListBySiteQuery) -> CmsResult<CmsContentTypePage> { unimplemented!() }
    async fn create_content_type(&self, _ctx: &CmsRequestContext, _command: ContentTypeCommand) -> CmsResult<CmsContentType> { unimplemented!() }
    async fn retrieve_content_type(&self, _ctx: &CmsRequestContext, _content_type_id: CmsId) -> CmsResult<CmsContentType> { unimplemented!() }
    async fn update_content_type(&self, _ctx: &CmsRequestContext, _content_type_id: CmsId, _command: ContentTypeCommand) -> CmsResult<CmsContentType> { unimplemented!() }
    async fn delete_content_type(&self, _ctx: &CmsRequestContext, _content_type_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_content_fields(&self, _ctx: &CmsRequestContext, _query: ListContentFieldsQuery) -> CmsResult<CmsContentFieldPage> { unimplemented!() }
    async fn create_content_field(&self, _ctx: &CmsRequestContext, _command: ContentFieldCommand) -> CmsResult<CmsContentField> { unimplemented!() }
    async fn update_content_field(&self, _ctx: &CmsRequestContext, _field_id: CmsId, _command: ContentFieldCommand) -> CmsResult<CmsContentField> { unimplemented!() }
    async fn delete_content_field(&self, _ctx: &CmsRequestContext, _field_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_taxonomies(&self, _ctx: &CmsRequestContext, _query: ListBySiteQuery) -> CmsResult<CmsTaxonomyPage> { unimplemented!() }
    async fn create_taxonomy(&self, _ctx: &CmsRequestContext, _command: TaxonomyCommand) -> CmsResult<CmsTaxonomy> { unimplemented!() }
    async fn update_taxonomy(&self, _ctx: &CmsRequestContext, _taxonomy_id: CmsId, _command: TaxonomyCommand) -> CmsResult<CmsTaxonomy> { unimplemented!() }
    async fn delete_taxonomy(&self, _ctx: &CmsRequestContext, _taxonomy_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_taxonomy_terms(&self, _ctx: &CmsRequestContext, _query: ListTaxonomyTermsQuery) -> CmsResult<CmsTaxonomyTermPage> { unimplemented!() }
    async fn create_taxonomy_term(&self, _ctx: &CmsRequestContext, _command: TaxonomyTermCommand) -> CmsResult<CmsTaxonomyTerm> { unimplemented!() }
    async fn update_taxonomy_term(&self, _ctx: &CmsRequestContext, _term_id: CmsId, _command: TaxonomyTermCommand) -> CmsResult<CmsTaxonomyTerm> { unimplemented!() }
    async fn delete_taxonomy_term(&self, _ctx: &CmsRequestContext, _term_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn create_entry(&self, _ctx: &CmsRequestContext, _command: EntryCommand) -> CmsResult<CmsEntry> { unimplemented!() }
    async fn update_entry(&self, _ctx: &CmsRequestContext, _entry_id: CmsId, _command: EntryCommand) -> CmsResult<CmsEntry> { unimplemented!() }
    async fn delete_entry(&self, _ctx: &CmsRequestContext, _entry_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn retrieve_entry(&self, _ctx: &CmsRequestContext, _entry_id: CmsId) -> CmsResult<CmsEntry> { unimplemented!() }
    async fn list_entries(&self, _ctx: &CmsRequestContext, _query: ListEntriesQuery) -> CmsResult<CmsEntryPage> { unimplemented!() }
    async fn replace_entry_body(&self, _ctx: &CmsRequestContext, _command: EntryBodyCommand) -> CmsResult<CmsEntry> { unimplemented!() }
    async fn replace_entry_fields(&self, _ctx: &CmsRequestContext, _command: EntryFieldsCommand) -> CmsResult<CmsEntry> { unimplemented!() }
    async fn list_entry_media(&self, _ctx: &CmsRequestContext, _query: ListEntryMediaQuery) -> CmsResult<CmsMediaRefPage> { unimplemented!() }
    async fn attach_entry_media(&self, _ctx: &CmsRequestContext, _command: EntryMediaCommand) -> CmsResult<CmsMediaRef> { unimplemented!() }
    async fn delete_entry_media(&self, _ctx: &CmsRequestContext, _media_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn replace_entry_terms(&self, _ctx: &CmsRequestContext, _command: ReplaceEntryTermsCommand) -> CmsResult<CmsEntry> { unimplemented!() }
    async fn list_entry_versions(&self, _ctx: &CmsRequestContext, _query: ListEntryVersionsQuery) -> CmsResult<CmsEntryVersionPage> { unimplemented!() }
    async fn create_publish_snapshot(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> { unimplemented!() }
    async fn publish_entry(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> { unimplemented!() }
    async fn unpublish_entry(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> { unimplemented!() }
    async fn rollback_entry(&self, _ctx: &CmsRequestContext, _command: RollbackCommand) -> CmsResult<CmsPublishSnapshot> { unimplemented!() }
    async fn schedule_entry(&self, _ctx: &CmsRequestContext, _command: ScheduleCommand) -> CmsResult<CmsEntry> { unimplemented!() }
    async fn list_pages(&self, _ctx: &CmsRequestContext, _query: ListPagesQuery) -> CmsResult<CmsPagePage> { unimplemented!() }
    async fn create_page(&self, _ctx: &CmsRequestContext, _command: PageCommand) -> CmsResult<CmsPageModel> { unimplemented!() }
    async fn retrieve_page(&self, _ctx: &CmsRequestContext, _page_id: CmsId) -> CmsResult<CmsPageModel> { unimplemented!() }
    async fn update_page(&self, _ctx: &CmsRequestContext, _page_id: CmsId, _command: PageCommand) -> CmsResult<CmsPageModel> { unimplemented!() }
    async fn delete_page(&self, _ctx: &CmsRequestContext, _page_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn replace_page_blocks(&self, _ctx: &CmsRequestContext, _command: PageBlocksCommand) -> CmsResult<CmsPageModel> { unimplemented!() }
    async fn publish_page(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> { unimplemented!() }
    async fn list_feeds(&self, _ctx: &CmsRequestContext, _query: ListFeedsQuery) -> CmsResult<CmsFeedPage> { unimplemented!() }
    async fn create_feed(&self, _ctx: &CmsRequestContext, _command: FeedCommand) -> CmsResult<CmsFeed> { unimplemented!() }
    async fn retrieve_feed(&self, _ctx: &CmsRequestContext, _feed_id: CmsId) -> CmsResult<CmsFeed> { unimplemented!() }
    async fn update_feed(&self, _ctx: &CmsRequestContext, _feed_id: CmsId, _command: FeedCommand) -> CmsResult<CmsFeed> { unimplemented!() }
    async fn delete_feed(&self, _ctx: &CmsRequestContext, _feed_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_feed_rules(&self, _ctx: &CmsRequestContext, _query: ListFeedRulesQuery) -> CmsResult<CmsFeedRulePage> { unimplemented!() }
    async fn create_feed_rule(&self, _ctx: &CmsRequestContext, _command: FeedRuleCommand) -> CmsResult<CmsFeedRule> { unimplemented!() }
    async fn update_feed_rule(&self, _ctx: &CmsRequestContext, _rule_id: CmsId, _command: FeedRuleCommand) -> CmsResult<CmsFeedRule> { unimplemented!() }
    async fn delete_feed_rule(&self, _ctx: &CmsRequestContext, _rule_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_feed_items(&self, _ctx: &CmsRequestContext, _query: ListFeedItemsQuery) -> CmsResult<CmsFeedItemPage> { unimplemented!() }
    async fn upsert_feed_items(&self, _ctx: &CmsRequestContext, _command: FeedItemsCommand) -> CmsResult<CmsFeedItemPage> { unimplemented!() }
    async fn delete_feed_item(&self, _ctx: &CmsRequestContext, _item_id: CmsId) -> CmsResult<CommandResult> { unimplemented!() }
    async fn retrieve_feed_snapshot(&self, _ctx: &CmsRequestContext, _snapshot_id: CmsId) -> CmsResult<CmsFeedSnapshot> { unimplemented!() }
    async fn publish_feed(&self, _ctx: &CmsRequestContext, _command: PublishCommand) -> CmsResult<CmsPublishSnapshot> { unimplemented!() }
    async fn create_favorite(&self, _ctx: &CmsRequestContext, _command: FavoriteCommand) -> CmsResult<CmsFavorite> { unimplemented!() }
    async fn list_favorites(&self, _ctx: &CmsRequestContext, _query: ListFavoritesQuery) -> CmsResult<CmsFavoritePage> { unimplemented!() }
    async fn delete_favorite(&self, _ctx: &CmsRequestContext, _favorite_uuid: String) -> CmsResult<CommandResult> { unimplemented!() }
    async fn list_audit_logs(&self, _ctx: &CmsRequestContext, _query: ListAuditLogsQuery) -> CmsResult<CmsAuditLogPage> { unimplemented!() }
    async fn list_outbox_events(&self, _ctx: &CmsRequestContext, _query: ListOutboxEventsQuery) -> CmsResult<CmsOutboxEventPage> { unimplemented!() }
    async fn retry_outbox_event(&self, _ctx: &CmsRequestContext, _command: RetryOutboxEventCommand) -> CmsResult<CommandResult> { unimplemented!() }
    async fn create_outbox_event(&self, _ctx: &CmsRequestContext, _event: CmsOutboxEventDraft) -> CmsResult<CommandResult> { unimplemented!() }
}

struct MockAuthorizer;

#[async_trait]
impl CmsIamAuthorizer for MockAuthorizer {
    async fn require_permission(&self, _ctx: &CmsRequestContext, _permission: &'static str) -> CmsResult<()> {
        Ok(())
    }
}

struct MockEventPublisher;

#[async_trait]
impl CmsEventPublisher for MockEventPublisher {
    async fn enqueue(&self, _ctx: &CmsRequestContext, _event: CmsOutboxEventDraft) -> CmsResult<()> {
        Ok(())
    }
}

#[test]
fn cms_service_can_be_constructed() {
    let repository = Arc::new(MockRepository);
    let authorizer = Arc::new(MockAuthorizer);
    let event_publisher = Arc::new(MockEventPublisher);
    let _service = CmsService::new(repository, authorizer, event_publisher);
}
