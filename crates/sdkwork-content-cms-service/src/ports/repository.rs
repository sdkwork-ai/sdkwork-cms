use async_trait::async_trait;

use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::CmsResult;

#[async_trait]
pub trait CmsRepository: Send + Sync {
    async fn list_sites(&self, ctx: &CmsRequestContext, query: ListSitesQuery) -> CmsResult<CmsSitePage>;
    async fn create_site(&self, ctx: &CmsRequestContext, command: SiteCommand) -> CmsResult<CmsSite>;
    async fn retrieve_site(&self, ctx: &CmsRequestContext, site_id: CmsId) -> CmsResult<CmsSite>;
    async fn update_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
        command: SiteCommand,
    ) -> CmsResult<CmsSite>;
    async fn delete_site(&self, ctx: &CmsRequestContext, site_id: CmsId) -> CmsResult<CommandResult>;

    async fn list_channels(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsChannelPage>;
    async fn create_channel(
        &self,
        ctx: &CmsRequestContext,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel>;
    async fn update_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel>;
    async fn delete_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
    ) -> CmsResult<CommandResult>;

    async fn list_content_types(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsContentTypePage>;
    async fn create_content_type(
        &self,
        ctx: &CmsRequestContext,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType>;
    async fn retrieve_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CmsContentType>;
    async fn update_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType>;
    async fn delete_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CommandResult>;

    async fn list_content_fields(
        &self,
        ctx: &CmsRequestContext,
        query: ListContentFieldsQuery,
    ) -> CmsResult<CmsContentFieldPage>;
    async fn create_content_field(
        &self,
        ctx: &CmsRequestContext,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField>;
    async fn update_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField>;
    async fn delete_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
    ) -> CmsResult<CommandResult>;

    async fn list_taxonomies(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsTaxonomyPage>;
    async fn create_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy>;
    async fn update_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy>;
    async fn delete_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
    ) -> CmsResult<CommandResult>;

    async fn list_taxonomy_terms(
        &self,
        ctx: &CmsRequestContext,
        query: ListTaxonomyTermsQuery,
    ) -> CmsResult<CmsTaxonomyTermPage>;
    async fn create_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm>;
    async fn update_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm>;
    async fn delete_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
    ) -> CmsResult<CommandResult>;

    async fn create_entry(&self, ctx: &CmsRequestContext, command: EntryCommand) -> CmsResult<CmsEntry>;
    async fn update_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
        command: EntryCommand,
    ) -> CmsResult<CmsEntry>;
    async fn delete_entry(&self, ctx: &CmsRequestContext, entry_id: CmsId) -> CmsResult<CommandResult>;
    async fn retrieve_entry(&self, ctx: &CmsRequestContext, entry_id: CmsId) -> CmsResult<CmsEntry>;
    async fn list_entries(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntriesQuery,
    ) -> CmsResult<CmsEntryPage>;

    async fn replace_entry_body(
        &self,
        ctx: &CmsRequestContext,
        command: EntryBodyCommand,
    ) -> CmsResult<CmsEntry>;
    async fn replace_entry_fields(
        &self,
        ctx: &CmsRequestContext,
        command: EntryFieldsCommand,
    ) -> CmsResult<CmsEntry>;
    async fn list_entry_media(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryMediaQuery,
    ) -> CmsResult<CmsMediaRefPage>;
    async fn attach_entry_media(
        &self,
        ctx: &CmsRequestContext,
        command: EntryMediaCommand,
    ) -> CmsResult<CmsMediaRef>;
    async fn delete_entry_media(
        &self,
        ctx: &CmsRequestContext,
        media_id: CmsId,
    ) -> CmsResult<CommandResult>;
    async fn replace_entry_terms(
        &self,
        ctx: &CmsRequestContext,
        command: ReplaceEntryTermsCommand,
    ) -> CmsResult<CmsEntry>;
    async fn list_entry_versions(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryVersionsQuery,
    ) -> CmsResult<CmsEntryVersionPage>;

    async fn create_publish_snapshot(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot>;
    async fn publish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot>;
    async fn unpublish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot>;
    async fn rollback_entry(
        &self,
        ctx: &CmsRequestContext,
        command: RollbackCommand,
    ) -> CmsResult<CmsPublishSnapshot>;
    async fn schedule_entry(
        &self,
        ctx: &CmsRequestContext,
        command: ScheduleCommand,
    ) -> CmsResult<CmsEntry>;

    async fn list_pages(&self, ctx: &CmsRequestContext, query: ListPagesQuery) -> CmsResult<CmsPagePage>;
    async fn create_page(&self, ctx: &CmsRequestContext, command: PageCommand)
        -> CmsResult<CmsPageModel>;
    async fn retrieve_page(&self, ctx: &CmsRequestContext, page_id: CmsId) -> CmsResult<CmsPageModel>;
    async fn update_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
        command: PageCommand,
    ) -> CmsResult<CmsPageModel>;
    async fn delete_page(&self, ctx: &CmsRequestContext, page_id: CmsId) -> CmsResult<CommandResult>;
    async fn replace_page_blocks(
        &self,
        ctx: &CmsRequestContext,
        command: PageBlocksCommand,
    ) -> CmsResult<CmsPageModel>;
    async fn publish_page(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot>;

    async fn list_feeds(&self, ctx: &CmsRequestContext, query: ListFeedsQuery) -> CmsResult<CmsFeedPage>;
    async fn create_feed(&self, ctx: &CmsRequestContext, command: FeedCommand) -> CmsResult<CmsFeed>;
    async fn retrieve_feed(&self, ctx: &CmsRequestContext, feed_id: CmsId) -> CmsResult<CmsFeed>;
    async fn update_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
        command: FeedCommand,
    ) -> CmsResult<CmsFeed>;
    async fn delete_feed(&self, ctx: &CmsRequestContext, feed_id: CmsId) -> CmsResult<CommandResult>;
    async fn list_feed_rules(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedRulesQuery,
    ) -> CmsResult<CmsFeedRulePage>;
    async fn create_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule>;
    async fn update_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        rule_id: CmsId,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule>;
    async fn delete_feed_rule(&self, ctx: &CmsRequestContext, rule_id: CmsId)
        -> CmsResult<CommandResult>;
    async fn list_feed_items(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedItemsQuery,
    ) -> CmsResult<CmsFeedItemPage>;
    async fn upsert_feed_items(
        &self,
        ctx: &CmsRequestContext,
        command: FeedItemsCommand,
    ) -> CmsResult<CmsFeedItemPage>;
    async fn delete_feed_item(&self, ctx: &CmsRequestContext, item_id: CmsId)
        -> CmsResult<CommandResult>;
    async fn retrieve_feed_snapshot(
        &self,
        ctx: &CmsRequestContext,
        snapshot_id: CmsId,
    ) -> CmsResult<CmsFeedSnapshot>;
    async fn publish_feed(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot>;

    async fn create_favorite(
        &self,
        ctx: &CmsRequestContext,
        command: FavoriteCommand,
    ) -> CmsResult<CmsFavorite>;
    async fn list_favorites(
        &self,
        ctx: &CmsRequestContext,
        query: ListFavoritesQuery,
    ) -> CmsResult<CmsFavoritePage>;
    async fn delete_favorite(
        &self,
        ctx: &CmsRequestContext,
        favorite_uuid: String,
    ) -> CmsResult<CommandResult>;

    async fn list_audit_logs(
        &self,
        ctx: &CmsRequestContext,
        query: ListAuditLogsQuery,
    ) -> CmsResult<CmsAuditLogPage>;
    async fn list_outbox_events(
        &self,
        ctx: &CmsRequestContext,
        query: ListOutboxEventsQuery,
    ) -> CmsResult<CmsOutboxEventPage>;
    async fn retry_outbox_event(
        &self,
        ctx: &CmsRequestContext,
        command: RetryOutboxEventCommand,
    ) -> CmsResult<CommandResult>;
    async fn create_outbox_event(
        &self,
        ctx: &CmsRequestContext,
        event: CmsOutboxEventDraft,
    ) -> CmsResult<CommandResult>;
}
