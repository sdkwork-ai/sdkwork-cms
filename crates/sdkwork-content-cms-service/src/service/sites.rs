use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::CmsResult;
use crate::service::CmsService;

impl CmsService {
    pub async fn list_sites(
        &self,
        ctx: &CmsRequestContext,
        query: ListSitesQuery,
    ) -> CmsResult<CmsSitePage> {
        ctx.require_permission("cms.site.read")?;
        self.repository().list_sites(ctx, query).await
    }

    pub async fn create_site(
        &self,
        ctx: &CmsRequestContext,
        command: SiteCommand,
    ) -> CmsResult<CmsSite> {
        ctx.require_permission("cms.site.manage")?;
        let site = self.repository().create_site(ctx, command).await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "site".to_string(),
                    aggregate_id: site.id,
                    event_type: CmsEventType::SiteCreated,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "site_id": site.id,
                        "code": site.code,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        Ok(site)
    }

    pub async fn retrieve_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
    ) -> CmsResult<CmsSite> {
        ctx.require_permission("cms.site.read")?;
        self.repository().retrieve_site(ctx, site_id).await
    }

    pub async fn update_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
        command: SiteCommand,
    ) -> CmsResult<CmsSite> {
        ctx.require_permission("cms.site.manage")?;
        self.repository().update_site(ctx, site_id, command).await
    }

    pub async fn delete_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.site.manage")?;
        self.repository().delete_site(ctx, site_id).await
    }

    pub async fn list_channels(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsChannelPage> {
        ctx.require_permission("cms.channel.read")?;
        self.repository().list_channels(ctx, query).await
    }

    pub async fn create_channel(
        &self,
        ctx: &CmsRequestContext,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel> {
        ctx.require_permission("cms.channel.manage")?;
        self.repository().create_channel(ctx, command).await
    }

    pub async fn update_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel> {
        ctx.require_permission("cms.channel.manage")?;
        self.repository()
            .update_channel(ctx, channel_id, command)
            .await
    }

    pub async fn delete_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.channel.manage")?;
        self.repository().delete_channel(ctx, channel_id).await
    }
}
