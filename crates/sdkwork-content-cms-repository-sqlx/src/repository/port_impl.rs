use async_trait::async_trait;
use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;
use sdkwork_content_cms_service::ports::CmsRepository;

use super::CmsSqlxRepository;

#[async_trait]
impl CmsRepository for CmsSqlxRepository {
    async fn list_sites(&self, ctx: &CmsRequestContext, query: ListSitesQuery) -> CmsResult<CmsSitePage> {
        CmsSqlxRepository::list_sites(self, ctx, query).await
    }

    async fn create_site(&self, ctx: &CmsRequestContext, command: SiteCommand) -> CmsResult<CmsSite> {
        CmsSqlxRepository::create_site(self, ctx, command).await
    }

    async fn retrieve_site(&self, ctx: &CmsRequestContext, site_id: CmsId) -> CmsResult<CmsSite> {
        CmsSqlxRepository::retrieve_site(self, ctx, site_id).await
    }

    async fn update_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
        command: SiteCommand,
    ) -> CmsResult<CmsSite> {
        CmsSqlxRepository::update_site(self, ctx, site_id, command).await
    }

    async fn delete_site(&self, ctx: &CmsRequestContext, site_id: CmsId) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_site(self, ctx, site_id).await
    }

    async fn list_channels(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsChannelPage> {
        CmsSqlxRepository::list_channels(self, ctx, query).await
    }

    async fn create_channel(
        &self,
        ctx: &CmsRequestContext,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel> {
        CmsSqlxRepository::create_channel(self, ctx, command).await
    }

    async fn update_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel> {
        CmsSqlxRepository::update_channel(self, ctx, channel_id, command).await
    }

    async fn delete_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_channel(self, ctx, channel_id).await
    }

    async fn list_content_types(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsContentTypePage> {
        CmsSqlxRepository::list_content_types(self, ctx, query).await
    }

    async fn create_content_type(
        &self,
        ctx: &CmsRequestContext,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType> {
        CmsSqlxRepository::create_content_type(self, ctx, command).await
    }

    async fn retrieve_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CmsContentType> {
        CmsSqlxRepository::retrieve_content_type(self, ctx, content_type_id).await
    }

    async fn update_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType> {
        CmsSqlxRepository::update_content_type(self, ctx, content_type_id, command).await
    }

    async fn delete_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_content_type(self, ctx, content_type_id).await
    }

    async fn list_content_fields(
        &self,
        ctx: &CmsRequestContext,
        query: ListContentFieldsQuery,
    ) -> CmsResult<CmsContentFieldPage> {
        CmsSqlxRepository::list_content_fields(self, ctx, query).await
    }

    async fn create_content_field(
        &self,
        ctx: &CmsRequestContext,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField> {
        CmsSqlxRepository::create_content_field(self, ctx, command).await
    }

    async fn update_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField> {
        CmsSqlxRepository::update_content_field(self, ctx, field_id, command).await
    }

    async fn delete_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_content_field(self, ctx, field_id).await
    }

    async fn list_taxonomies(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsTaxonomyPage> {
        CmsSqlxRepository::list_taxonomies(self, ctx, query).await
    }

    async fn create_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy> {
        CmsSqlxRepository::create_taxonomy(self, ctx, command).await
    }

    async fn update_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy> {
        CmsSqlxRepository::update_taxonomy(self, ctx, taxonomy_id, command).await
    }

    async fn delete_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_taxonomy(self, ctx, taxonomy_id).await
    }

    async fn list_taxonomy_terms(
        &self,
        ctx: &CmsRequestContext,
        query: ListTaxonomyTermsQuery,
    ) -> CmsResult<CmsTaxonomyTermPage> {
        CmsSqlxRepository::list_taxonomy_terms(self, ctx, query).await
    }

    async fn create_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm> {
        CmsSqlxRepository::create_taxonomy_term(self, ctx, command).await
    }

    async fn update_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm> {
        CmsSqlxRepository::update_taxonomy_term(self, ctx, term_id, command).await
    }

    async fn delete_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_taxonomy_term(self, ctx, term_id).await
    }

    async fn create_entry(&self, ctx: &CmsRequestContext, command: EntryCommand) -> CmsResult<CmsEntry> {
        CmsSqlxRepository::create_entry(self, ctx, command).await
    }

    async fn update_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
        command: EntryCommand,
    ) -> CmsResult<CmsEntry> {
        CmsSqlxRepository::update_entry(self, ctx, entry_id, command).await
    }

    async fn delete_entry(&self, ctx: &CmsRequestContext, entry_id: CmsId) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_entry(self, ctx, entry_id).await
    }

    async fn retrieve_entry(&self, ctx: &CmsRequestContext, entry_id: CmsId) -> CmsResult<CmsEntry> {
        CmsSqlxRepository::retrieve_entry(self, ctx, entry_id).await
    }

    async fn list_entries(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntriesQuery,
    ) -> CmsResult<CmsEntryPage> {
        CmsSqlxRepository::list_entries(self, ctx, query).await
    }

    async fn replace_entry_body(
        &self,
        ctx: &CmsRequestContext,
        command: EntryBodyCommand,
    ) -> CmsResult<CmsEntry> {
        CmsSqlxRepository::replace_entry_body(self, ctx, command).await
    }

    async fn replace_entry_fields(
        &self,
        ctx: &CmsRequestContext,
        command: EntryFieldsCommand,
    ) -> CmsResult<CmsEntry> {
        CmsSqlxRepository::replace_entry_fields(self, ctx, command).await
    }

    async fn list_entry_media(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryMediaQuery,
    ) -> CmsResult<CmsMediaRefPage> {
        CmsSqlxRepository::list_entry_media(self, ctx, query).await
    }

    async fn attach_entry_media(
        &self,
        ctx: &CmsRequestContext,
        command: EntryMediaCommand,
    ) -> CmsResult<CmsMediaRef> {
        CmsSqlxRepository::attach_entry_media(self, ctx, command).await
    }

    async fn delete_entry_media(
        &self,
        ctx: &CmsRequestContext,
        media_id: CmsId,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_entry_media(self, ctx, media_id).await
    }

    async fn replace_entry_terms(
        &self,
        ctx: &CmsRequestContext,
        command: ReplaceEntryTermsCommand,
    ) -> CmsResult<CmsEntry> {
        CmsSqlxRepository::replace_entry_terms(self, ctx, command).await
    }

    async fn list_entry_versions(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryVersionsQuery,
    ) -> CmsResult<CmsEntryVersionPage> {
        CmsSqlxRepository::list_entry_versions(self, ctx, query).await
    }

    async fn create_publish_snapshot(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        CmsSqlxRepository::create_publish_snapshot(self, ctx, command).await
    }

    async fn publish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        CmsSqlxRepository::publish_entry(self, ctx, command).await
    }

    async fn unpublish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        CmsSqlxRepository::unpublish_entry(self, ctx, command).await
    }

    async fn rollback_entry(
        &self,
        ctx: &CmsRequestContext,
        command: RollbackCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        CmsSqlxRepository::rollback_entry(self, ctx, command).await
    }

    async fn schedule_entry(
        &self,
        ctx: &CmsRequestContext,
        command: ScheduleCommand,
    ) -> CmsResult<CmsEntry> {
        CmsSqlxRepository::schedule_entry(self, ctx, command).await
    }

    async fn list_pages(&self, ctx: &CmsRequestContext, query: ListPagesQuery) -> CmsResult<CmsPagePage> {
        CmsSqlxRepository::list_pages(self, ctx, query).await
    }

    async fn create_page(
        &self,
        ctx: &CmsRequestContext,
        command: PageCommand,
    ) -> CmsResult<CmsPageModel> {
        CmsSqlxRepository::create_page(self, ctx, command).await
    }

    async fn retrieve_page(&self, ctx: &CmsRequestContext, page_id: CmsId) -> CmsResult<CmsPageModel> {
        CmsSqlxRepository::retrieve_page(self, ctx, page_id).await
    }

    async fn update_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
        command: PageCommand,
    ) -> CmsResult<CmsPageModel> {
        CmsSqlxRepository::update_page(self, ctx, page_id, command).await
    }

    async fn delete_page(&self, ctx: &CmsRequestContext, page_id: CmsId) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_page(self, ctx, page_id).await
    }

    async fn replace_page_blocks(
        &self,
        ctx: &CmsRequestContext,
        command: PageBlocksCommand,
    ) -> CmsResult<CmsPageModel> {
        CmsSqlxRepository::replace_page_blocks(self, ctx, command).await
    }

    async fn publish_page(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        CmsSqlxRepository::publish_page(self, ctx, command).await
    }

    async fn list_feeds(&self, ctx: &CmsRequestContext, query: ListFeedsQuery) -> CmsResult<CmsFeedPage> {
        CmsSqlxRepository::list_feeds(self, ctx, query).await
    }

    async fn create_feed(&self, ctx: &CmsRequestContext, command: FeedCommand) -> CmsResult<CmsFeed> {
        CmsSqlxRepository::create_feed(self, ctx, command).await
    }

    async fn retrieve_feed(&self, ctx: &CmsRequestContext, feed_id: CmsId) -> CmsResult<CmsFeed> {
        CmsSqlxRepository::retrieve_feed(self, ctx, feed_id).await
    }

    async fn update_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
        command: FeedCommand,
    ) -> CmsResult<CmsFeed> {
        CmsSqlxRepository::update_feed(self, ctx, feed_id, command).await
    }

    async fn delete_feed(&self, ctx: &CmsRequestContext, feed_id: CmsId) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_feed(self, ctx, feed_id).await
    }

    async fn list_feed_rules(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedRulesQuery,
    ) -> CmsResult<CmsFeedRulePage> {
        CmsSqlxRepository::list_feed_rules(self, ctx, query).await
    }

    async fn create_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule> {
        CmsSqlxRepository::create_feed_rule(self, ctx, command).await
    }

    async fn update_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        rule_id: CmsId,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule> {
        CmsSqlxRepository::update_feed_rule(self, ctx, rule_id, command).await
    }

    async fn delete_feed_rule(&self, ctx: &CmsRequestContext, rule_id: CmsId)
        -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_feed_rule(self, ctx, rule_id).await
    }

    async fn list_feed_items(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedItemsQuery,
    ) -> CmsResult<CmsFeedItemPage> {
        CmsSqlxRepository::list_feed_items(self, ctx, query).await
    }

    async fn upsert_feed_items(
        &self,
        ctx: &CmsRequestContext,
        command: FeedItemsCommand,
    ) -> CmsResult<CmsFeedItemPage> {
        CmsSqlxRepository::upsert_feed_items(self, ctx, command).await
    }

    async fn delete_feed_item(&self, ctx: &CmsRequestContext, item_id: CmsId)
        -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_feed_item(self, ctx, item_id).await
    }

    async fn retrieve_feed_snapshot(
        &self,
        ctx: &CmsRequestContext,
        snapshot_id: CmsId,
    ) -> CmsResult<CmsFeedSnapshot> {
        CmsSqlxRepository::retrieve_feed_snapshot(self, ctx, snapshot_id).await
    }

    async fn publish_feed(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        CmsSqlxRepository::publish_feed(self, ctx, command).await
    }

    async fn create_favorite(
        &self,
        ctx: &CmsRequestContext,
        command: FavoriteCommand,
    ) -> CmsResult<CmsFavorite> {
        CmsSqlxRepository::create_favorite(self, ctx, command).await
    }

    async fn list_favorites(
        &self,
        ctx: &CmsRequestContext,
        query: ListFavoritesQuery,
    ) -> CmsResult<CmsFavoritePage> {
        CmsSqlxRepository::list_favorites(self, ctx, query).await
    }

    async fn delete_favorite(
        &self,
        ctx: &CmsRequestContext,
        favorite_uuid: String,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::delete_favorite(self, ctx, favorite_uuid).await
    }

    async fn list_audit_logs(
        &self,
        ctx: &CmsRequestContext,
        query: ListAuditLogsQuery,
    ) -> CmsResult<CmsAuditLogPage> {
        CmsSqlxRepository::list_audit_logs(self, ctx, query).await
    }

    async fn list_outbox_events(
        &self,
        ctx: &CmsRequestContext,
        query: ListOutboxEventsQuery,
    ) -> CmsResult<CmsOutboxEventPage> {
        CmsSqlxRepository::list_outbox_events(self, ctx, query).await
    }

    async fn retry_outbox_event(
        &self,
        ctx: &CmsRequestContext,
        command: RetryOutboxEventCommand,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::retry_outbox_event(self, ctx, command).await
    }

    async fn create_outbox_event(
        &self,
        ctx: &CmsRequestContext,
        event: CmsOutboxEventDraft,
    ) -> CmsResult<CommandResult> {
        CmsSqlxRepository::create_outbox_event(self, ctx, event).await
    }
}
