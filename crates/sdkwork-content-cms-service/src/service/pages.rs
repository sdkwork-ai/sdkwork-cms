use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::CmsResult;
use crate::service::CmsService;

impl CmsService {
    pub async fn list_pages(
        &self,
        ctx: &CmsRequestContext,
        query: ListPagesQuery,
    ) -> CmsResult<CmsPagePage> {
        ctx.require_permission("cms.page.read")?;
        self.repository().list_pages(ctx, query).await
    }

    pub async fn create_page(
        &self,
        ctx: &CmsRequestContext,
        command: PageCommand,
    ) -> CmsResult<CmsPageModel> {
        ctx.require_permission("cms.page.manage")?;
        self.repository().create_page(ctx, command).await
    }

    pub async fn retrieve_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
    ) -> CmsResult<CmsPageModel> {
        ctx.require_permission("cms.page.read")?;
        self.repository().retrieve_page(ctx, page_id).await
    }

    pub async fn update_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
        command: PageCommand,
    ) -> CmsResult<CmsPageModel> {
        ctx.require_permission("cms.page.manage")?;
        self.repository().update_page(ctx, page_id, command).await
    }

    pub async fn delete_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.page.manage")?;
        self.repository().delete_page(ctx, page_id).await
    }

    pub async fn replace_page_blocks(
        &self,
        ctx: &CmsRequestContext,
        command: PageBlocksCommand,
    ) -> CmsResult<CmsPageModel> {
        ctx.require_permission("cms.page.manage")?;
        self.repository().replace_page_blocks(ctx, command).await
    }

    pub async fn publish_page(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        ctx.require_permission("cms.page.publish")?;
        let snapshot = self.repository().publish_page(ctx, command.clone()).await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "page".to_string(),
                    aggregate_id: command.owner_id,
                    event_type: CmsEventType::PagePublished,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "page_id": command.owner_id,
                        "snapshot_id": snapshot.id,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        if let Some(port) = self.cache_invalidation_port() {
            let _ = port
                .request_cache_invalidation(ctx, &format!("page:{}", command.owner_id))
                .await;
        }
        if let Some(port) = self.sitemap_projection_port() {
            let _ = port.request_sitemap_projection(ctx, 0).await;
        }
        Ok(snapshot)
    }
}
