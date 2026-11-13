use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::CmsResult;
use crate::service::CmsService;

impl CmsService {
    pub async fn list_feeds(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedsQuery,
    ) -> CmsResult<CmsFeedPage> {
        ctx.require_permission("cms.feed.read")?;
        self.repository().list_feeds(ctx, query).await
    }

    pub async fn create_feed(
        &self,
        ctx: &CmsRequestContext,
        command: FeedCommand,
    ) -> CmsResult<CmsFeed> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository().create_feed(ctx, command).await
    }

    pub async fn retrieve_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
    ) -> CmsResult<CmsFeed> {
        ctx.require_permission("cms.feed.read")?;
        self.repository().retrieve_feed(ctx, feed_id).await
    }

    pub async fn update_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
        command: FeedCommand,
    ) -> CmsResult<CmsFeed> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository().update_feed(ctx, feed_id, command).await
    }

    pub async fn delete_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository().delete_feed(ctx, feed_id).await
    }

    pub async fn list_feed_rules(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedRulesQuery,
    ) -> CmsResult<CmsFeedRulePage> {
        ctx.require_permission("cms.feed.read")?;
        self.repository().list_feed_rules(ctx, query).await
    }

    pub async fn create_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository().create_feed_rule(ctx, command).await
    }

    pub async fn update_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        rule_id: CmsId,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository()
            .update_feed_rule(ctx, rule_id, command)
            .await
    }

    pub async fn delete_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        rule_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository().delete_feed_rule(ctx, rule_id).await
    }

    pub async fn list_feed_items(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedItemsQuery,
    ) -> CmsResult<CmsFeedItemPage> {
        ctx.require_permission("cms.feed.read")?;
        self.repository().list_feed_items(ctx, query).await
    }

    pub async fn upsert_feed_items(
        &self,
        ctx: &CmsRequestContext,
        command: FeedItemsCommand,
    ) -> CmsResult<CmsFeedItemPage> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository().upsert_feed_items(ctx, command).await
    }

    pub async fn delete_feed_item(
        &self,
        ctx: &CmsRequestContext,
        item_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.feed.manage")?;
        self.repository().delete_feed_item(ctx, item_id).await
    }

    pub async fn publish_feed(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        ctx.require_permission("cms.feed.publish")?;

        let feed = self
            .repository()
            .retrieve_feed(ctx, command.owner_id)
            .await?;

        let _composed = self
            .compose_feed(ctx, feed.id, feed.feed_kind.as_str())
            .await?;

        let snapshot = self.repository().publish_feed(ctx, command.clone()).await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "feed".to_string(),
                    aggregate_id: command.owner_id,
                    event_type: CmsEventType::FeedPublished,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "feed_id": command.owner_id,
                        "snapshot_id": snapshot.id,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        if let Some(port) = self.cache_invalidation_port() {
            let _ = port
                .request_cache_invalidation(ctx, &format!("feed:{}", command.owner_id))
                .await;
        }
        Ok(snapshot)
    }

    pub async fn retrieve_feed_snapshot(
        &self,
        ctx: &CmsRequestContext,
        snapshot_id: CmsId,
    ) -> CmsResult<CmsFeedSnapshot> {
        ctx.require_permission("cms.feed.read")?;
        self.repository()
            .retrieve_feed_snapshot(ctx, snapshot_id)
            .await
    }

    pub async fn compose_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
        feed_kind: &str,
    ) -> CmsResult<Vec<CmsFeedItem>> {
        let mut all_items: Vec<CmsFeedItem> = Vec::new();
        let mut seen_ids: std::collections::HashSet<CmsId> = std::collections::HashSet::new();

        let curated_items = self
            .repository()
            .list_feed_items(
                ctx,
                ListFeedItemsQuery {
                    feed_id,
                    status: Some(1),
                    cursor: None,
                    limit: 1000,
                },
            )
            .await?;

        for item in &curated_items.items {
            if item.pinned {
                let key = item.entry_id.or(item.page_id).unwrap_or(0);
                if seen_ids.insert(key) {
                    all_items.push(item.clone());
                }
            }
        }

        let rules = self
            .repository()
            .list_feed_rules(
                ctx,
                ListFeedRulesQuery {
                    feed_id,
                    enabled: Some(true),
                    cursor: None,
                    limit: 100,
                },
            )
            .await?;

        for rule in rules.items {
            match rule.rule_kind.as_str() {
                "curated" => {
                    for item in &curated_items.items {
                        if !item.pinned {
                            let key = item.entry_id.or(item.page_id).unwrap_or(0);
                            if seen_ids.insert(key) {
                                all_items.push(item.clone());
                            }
                        }
                    }
                }
                "rule" => {
                    let entries = self
                        .repository()
                        .list_entries(
                            ctx,
                            ListEntriesQuery {
                                site_id: None,
                                content_type_id: None,
                                channel_id: None,
                                locale: None,
                                entry_status: None,
                                publication_status: Some(20),
                                author_user_id: None,
                                cursor: None,
                                limit: 50,
                            },
                        )
                        .await?;

                    for entry in entries.items {
                        if seen_ids.insert(entry.id) {
                            all_items.push(CmsFeedItem {
                                id: 0,
                                feed_id,
                                entry_id: Some(entry.id),
                                page_id: None,
                                external_url: None,
                                item_kind: "entry".to_string(),
                                pinned: false,
                                sort_order: all_items.len() as i32,
                            });
                        }
                    }
                }
                "hybrid" => {
                    for item in &curated_items.items {
                        if !item.pinned {
                            let key = item.entry_id.or(item.page_id).unwrap_or(0);
                            if seen_ids.insert(key) {
                                all_items.push(item.clone());
                            }
                        }
                    }

                    let remaining = 50u32.saturating_sub(all_items.len() as u32);
                    if remaining > 0 {
                        let entries = self
                            .repository()
                            .list_entries(
                                ctx,
                                ListEntriesQuery {
                                    site_id: None,
                                    content_type_id: None,
                                    channel_id: None,
                                    locale: None,
                                    entry_status: None,
                                    publication_status: Some(20),
                                    author_user_id: None,
                                    cursor: None,
                                    limit: remaining,
                                },
                            )
                            .await?;

                        for entry in entries.items {
                            if seen_ids.insert(entry.id) {
                                all_items.push(CmsFeedItem {
                                    id: 0,
                                    feed_id,
                                    entry_id: Some(entry.id),
                                    page_id: None,
                                    external_url: None,
                                    item_kind: "entry".to_string(),
                                    pinned: false,
                                    sort_order: all_items.len() as i32,
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        match feed_kind {
            "curated" => {
                let mut final_items: Vec<CmsFeedItem> = Vec::new();
                for item in &curated_items.items {
                    let key = item.entry_id.or(item.page_id).unwrap_or(0);
                    if seen_ids.contains(&key) {
                        final_items.push(item.clone());
                    }
                }
                Ok(final_items)
            }
            _ => Ok(all_items),
        }
    }
}
