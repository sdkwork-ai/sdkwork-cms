use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::{CmsError, CmsResult};
use crate::service::CmsService;

impl CmsService {
    pub async fn delivery_list_entries(
        &self,
        ctx: &CmsRequestContext,
        query: DeliveryListEntriesQuery,
    ) -> CmsResult<CmsEntryPage> {
        self.repository()
            .list_entries(
                ctx,
                ListEntriesQuery {
                    site_id: None,
                    content_type_id: None,
                    channel_id: None,
                    locale: query.locale,
                    entry_status: None,
                    publication_status: Some(20),
                    author_user_id: None,
                    cursor: query.cursor,
                    limit: query.limit,
                },
            )
            .await
    }

    pub async fn delivery_bootstrap(
        &self,
        ctx: &CmsRequestContext,
        query: DeliveryBootstrapQuery,
    ) -> CmsResult<CmsDeliveryBootstrap> {
        let sites = self
            .repository()
            .list_sites(
                ctx,
                ListSitesQuery {
                    cursor: None,
                    limit: 1000,
                },
            )
            .await?;
        let site = sites
            .items
            .into_iter()
            .find(|s| s.code == query.site_code)
            .ok_or(CmsError::not_found("site"))?;

        let channels = self
            .repository()
            .list_channels(
                ctx,
                ListBySiteQuery {
                    site_id: site.id,
                    cursor: None,
                    limit: 100,
                },
            )
            .await?;

        Ok(CmsDeliveryBootstrap {
            site,
            channels: channels.items,
        })
    }

    pub async fn delivery_resolve_entry(
        &self,
        ctx: &CmsRequestContext,
        query: DeliveryResolveEntryQuery,
    ) -> CmsResult<CmsEntry> {
        if let Some(ref token) = query.preview_token {
            if let Some(port) = self.preview_token_port() {
                port.validate_preview_token(ctx, token, "entry", 0).await?;
            }
        }

        let entries = self
            .repository()
            .list_entries(
                ctx,
                ListEntriesQuery {
                    site_id: None,
                    content_type_id: None,
                    channel_id: None,
                    locale: query.locale,
                    entry_status: None,
                    publication_status: Some(20),
                    author_user_id: None,
                    cursor: None,
                    limit: 100,
                },
            )
            .await?;

        entries
            .items
            .into_iter()
            .find(|e| e.slug == query.slug)
            .ok_or(CmsError::not_found("entry"))
    }

    pub async fn delivery_retrieve_entry(
        &self,
        ctx: &CmsRequestContext,
        query: DeliveryRetrieveEntryQuery,
    ) -> CmsResult<CmsEntry> {
        if let Some(ref token) = query.preview_token {
            if let Some(port) = self.preview_token_port() {
                port.validate_preview_token(ctx, token, "entry", query.entry_id)
                    .await?;
            }
        }
        self.repository().retrieve_entry(ctx, query.entry_id).await
    }

    pub async fn delivery_resolve_page(
        &self,
        ctx: &CmsRequestContext,
        query: DeliveryResolvePageQuery,
    ) -> CmsResult<CmsPageModel> {
        if let Some(ref token) = query.preview_token {
            if let Some(port) = self.preview_token_port() {
                port.validate_preview_token(ctx, token, "page", 0).await?;
            }
        }

        let pages = self
            .repository()
            .list_pages(
                ctx,
                ListPagesQuery {
                    site_id: None,
                    channel_id: None,
                    locale: query.locale,
                    status: None,
                    cursor: None,
                    limit: 100,
                },
            )
            .await?;

        pages
            .items
            .into_iter()
            .find(|p| p.path == query.path)
            .ok_or(CmsError::not_found("page"))
    }

    pub async fn delivery_list_feed_items(
        &self,
        ctx: &CmsRequestContext,
        query: DeliveryFeedItemsQuery,
    ) -> CmsResult<CmsFeedItemPage> {
        let feeds = self
            .repository()
            .list_feeds(
                ctx,
                ListFeedsQuery {
                    site_id: None,
                    channel_id: None,
                    locale: query.locale,
                    status: None,
                    cursor: None,
                    limit: 100,
                },
            )
            .await?;

        let feed = feeds
            .items
            .into_iter()
            .find(|f| f.code == query.feed_code)
            .ok_or(CmsError::not_found("feed"))?;

        self.repository()
            .list_feed_items(
                ctx,
                ListFeedItemsQuery {
                    feed_id: feed.id,
                    status: None,
                    cursor: query.cursor,
                    limit: query.limit,
                },
            )
            .await
    }
}
