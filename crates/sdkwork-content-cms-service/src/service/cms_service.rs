use std::sync::Arc;

use crate::ports::{
    CmsCacheInvalidationPort, CmsDriveMediaPort, CmsEngagementPort, CmsEventPublisher,
    CmsIamAuthorizer, CmsNotificationPort, CmsPreviewTokenPort, CmsRepository, CmsSchedulerPort,
    CmsSearchSyncPort, CmsSitemapProjectionPort, CmsWebhookPort,
};

#[derive(Clone)]
pub struct CmsService {
    repository: Arc<dyn CmsRepository + Send + Sync>,
    authorizer: Arc<dyn CmsIamAuthorizer + Send + Sync>,
    event_publisher: Arc<dyn CmsEventPublisher + Send + Sync>,
    drive_media_port: Option<Arc<dyn CmsDriveMediaPort + Send + Sync>>,
    search_sync_port: Option<Arc<dyn CmsSearchSyncPort + Send + Sync>>,
    cache_invalidation_port: Option<Arc<dyn CmsCacheInvalidationPort + Send + Sync>>,
    preview_token_port: Option<Arc<dyn CmsPreviewTokenPort + Send + Sync>>,
    scheduler_port: Option<Arc<dyn CmsSchedulerPort + Send + Sync>>,
    notification_port: Option<Arc<dyn CmsNotificationPort + Send + Sync>>,
    webhook_port: Option<Arc<dyn CmsWebhookPort + Send + Sync>>,
    engagement_port: Option<Arc<dyn CmsEngagementPort + Send + Sync>>,
    sitemap_projection_port: Option<Arc<dyn CmsSitemapProjectionPort + Send + Sync>>,
}

impl CmsService {
    pub fn new(
        repository: Arc<dyn CmsRepository + Send + Sync>,
        authorizer: Arc<dyn CmsIamAuthorizer + Send + Sync>,
        event_publisher: Arc<dyn CmsEventPublisher + Send + Sync>,
    ) -> Self {
        Self {
            repository,
            authorizer,
            event_publisher,
            drive_media_port: None,
            search_sync_port: None,
            cache_invalidation_port: None,
            preview_token_port: None,
            scheduler_port: None,
            notification_port: None,
            webhook_port: None,
            engagement_port: None,
            sitemap_projection_port: None,
        }
    }

    pub fn with_drive_media_port(mut self, port: Arc<dyn CmsDriveMediaPort + Send + Sync>) -> Self {
        self.drive_media_port = Some(port);
        self
    }

    pub fn with_search_sync_port(mut self, port: Arc<dyn CmsSearchSyncPort + Send + Sync>) -> Self {
        self.search_sync_port = Some(port);
        self
    }

    pub fn with_cache_invalidation_port(
        mut self,
        port: Arc<dyn CmsCacheInvalidationPort + Send + Sync>,
    ) -> Self {
        self.cache_invalidation_port = Some(port);
        self
    }

    pub fn with_preview_token_port(
        mut self,
        port: Arc<dyn CmsPreviewTokenPort + Send + Sync>,
    ) -> Self {
        self.preview_token_port = Some(port);
        self
    }

    pub fn with_scheduler_port(mut self, port: Arc<dyn CmsSchedulerPort + Send + Sync>) -> Self {
        self.scheduler_port = Some(port);
        self
    }

    pub fn with_notification_port(
        mut self,
        port: Arc<dyn CmsNotificationPort + Send + Sync>,
    ) -> Self {
        self.notification_port = Some(port);
        self
    }

    pub fn with_webhook_port(mut self, port: Arc<dyn CmsWebhookPort + Send + Sync>) -> Self {
        self.webhook_port = Some(port);
        self
    }

    pub fn with_engagement_port(mut self, port: Arc<dyn CmsEngagementPort + Send + Sync>) -> Self {
        self.engagement_port = Some(port);
        self
    }

    pub fn with_sitemap_projection_port(
        mut self,
        port: Arc<dyn CmsSitemapProjectionPort + Send + Sync>,
    ) -> Self {
        self.sitemap_projection_port = Some(port);
        self
    }

    pub fn repository(&self) -> &(dyn CmsRepository + Send + Sync) {
        self.repository.as_ref()
    }

    pub fn authorizer(&self) -> &(dyn CmsIamAuthorizer + Send + Sync) {
        self.authorizer.as_ref()
    }

    pub fn event_publisher(&self) -> &(dyn CmsEventPublisher + Send + Sync) {
        self.event_publisher.as_ref()
    }

    pub fn search_sync_port(&self) -> Option<&(dyn CmsSearchSyncPort + Send + Sync)> {
        self.search_sync_port.as_ref().map(|p| p.as_ref())
    }

    pub fn cache_invalidation_port(&self) -> Option<&(dyn CmsCacheInvalidationPort + Send + Sync)> {
        self.cache_invalidation_port.as_ref().map(|p| p.as_ref())
    }

    pub fn preview_token_port(&self) -> Option<&(dyn CmsPreviewTokenPort + Send + Sync)> {
        self.preview_token_port.as_ref().map(|p| p.as_ref())
    }

    pub fn scheduler_port(&self) -> Option<&(dyn CmsSchedulerPort + Send + Sync)> {
        self.scheduler_port.as_ref().map(|p| p.as_ref())
    }

    pub fn notification_port(&self) -> Option<&(dyn CmsNotificationPort + Send + Sync)> {
        self.notification_port.as_ref().map(|p| p.as_ref())
    }

    pub fn webhook_port(&self) -> Option<&(dyn CmsWebhookPort + Send + Sync)> {
        self.webhook_port.as_ref().map(|p| p.as_ref())
    }

    pub fn sitemap_projection_port(&self) -> Option<&(dyn CmsSitemapProjectionPort + Send + Sync)> {
        self.sitemap_projection_port.as_ref().map(|p| p.as_ref())
    }

    pub fn engagement_port(&self) -> Option<&(dyn CmsEngagementPort + Send + Sync)> {
        self.engagement_port.as_ref().map(|p| p.as_ref())
    }
}
