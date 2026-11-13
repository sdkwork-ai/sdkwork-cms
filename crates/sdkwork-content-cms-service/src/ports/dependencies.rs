use async_trait::async_trait;

use crate::context::CmsRequestContext;
use crate::domain::{CmsId, CmsJson};
use crate::error::CmsResult;

#[async_trait]
pub trait CmsIamAuthorizer: Send + Sync {
    async fn require_permission(
        &self,
        ctx: &CmsRequestContext,
        permission: &'static str,
    ) -> CmsResult<()>;
}

#[async_trait]
pub trait CmsDriveMediaPort: Send + Sync {
    async fn validate_media_reference(
        &self,
        ctx: &CmsRequestContext,
        drive_space_id: Option<&str>,
        drive_node_id: Option<&str>,
        drive_uri: Option<&str>,
        media_resource_id: Option<&str>,
    ) -> CmsResult<CmsJson>;
}

#[async_trait]
pub trait CmsSearchSyncPort: Send + Sync {
    async fn request_search_sync(
        &self,
        ctx: &CmsRequestContext,
        owner_type: &str,
        owner_id: CmsId,
    ) -> CmsResult<()>;
}

#[async_trait]
pub trait CmsCacheInvalidationPort: Send + Sync {
    async fn request_cache_invalidation(
        &self,
        ctx: &CmsRequestContext,
        cache_tags_json: &str,
    ) -> CmsResult<()>;
}

#[async_trait]
pub trait CmsPreviewTokenPort: Send + Sync {
    async fn validate_preview_token(
        &self,
        ctx: &CmsRequestContext,
        token: &str,
        owner_type: &str,
        owner_id: CmsId,
    ) -> CmsResult<()>;
}

#[async_trait]
pub trait CmsSchedulerPort: Send + Sync {
    async fn schedule_publish_job(
        &self,
        ctx: &CmsRequestContext,
        owner_type: &str,
        owner_id: CmsId,
        run_at: &str,
    ) -> CmsResult<()>;

    async fn schedule_unpublish_job(
        &self,
        ctx: &CmsRequestContext,
        owner_type: &str,
        owner_id: CmsId,
        run_at: &str,
    ) -> CmsResult<()>;
}

#[async_trait]
pub trait CmsNotificationPort: Send + Sync {
    async fn notify_workflow_subscribers(
        &self,
        ctx: &CmsRequestContext,
        event_type: &str,
        payload_json: &str,
    ) -> CmsResult<()>;
}

#[async_trait]
pub trait CmsWebhookPort: Send + Sync {
    async fn enqueue_webhook_delivery(
        &self,
        ctx: &CmsRequestContext,
        event_type: &str,
        payload_json: &str,
    ) -> CmsResult<()>;
}

#[async_trait]
pub trait CmsEngagementPort: Send + Sync {
    async fn summarize_engagement(
        &self,
        ctx: &CmsRequestContext,
        owner_type: &str,
        owner_id: CmsId,
    ) -> CmsResult<CmsJson>;
}

#[async_trait]
pub trait CmsSitemapProjectionPort: Send + Sync {
    async fn request_sitemap_projection(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
    ) -> CmsResult<()>;

    async fn request_rss_projection(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
    ) -> CmsResult<()>;
}
