use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::CmsResult;
use crate::service::CmsService;

impl CmsService {
    pub async fn list_entries(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntriesQuery,
    ) -> CmsResult<CmsEntryPage> {
        ctx.require_permission("cms.entry.read")?;
        self.repository().list_entries(ctx, query).await
    }

    pub async fn create_entry(
        &self,
        ctx: &CmsRequestContext,
        command: EntryCommand,
    ) -> CmsResult<CmsEntry> {
        ctx.require_permission("cms.entry.create")?;
        let entry = self.repository().create_entry(ctx, command).await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "entry".to_string(),
                    aggregate_id: entry.id,
                    event_type: CmsEventType::EntryCreated,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "entry_id": entry.id,
                        "site_id": entry.site_id,
                        "title": entry.title,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        Ok(entry)
    }

    pub async fn retrieve_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
    ) -> CmsResult<CmsEntry> {
        ctx.require_permission("cms.entry.read")?;
        self.repository().retrieve_entry(ctx, entry_id).await
    }

    pub async fn update_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
        command: EntryCommand,
    ) -> CmsResult<CmsEntry> {
        ctx.require_permission("cms.entry.update")?;
        let entry = self
            .repository()
            .update_entry(ctx, entry_id, command)
            .await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "entry".to_string(),
                    aggregate_id: entry.id,
                    event_type: CmsEventType::EntryUpdated,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "entry_id": entry.id,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        Ok(entry)
    }

    pub async fn delete_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.entry.delete")?;
        let result = self.repository().delete_entry(ctx, entry_id).await?;
        if let Some(port) = self.search_sync_port() {
            let _ = port.request_search_sync(ctx, "entry", entry_id).await;
        }
        if let Some(port) = self.cache_invalidation_port() {
            let _ = port
                .request_cache_invalidation(ctx, &format!("entry:{}", entry_id))
                .await;
        }
        Ok(result)
    }

    pub async fn replace_entry_body(
        &self,
        ctx: &CmsRequestContext,
        command: EntryBodyCommand,
    ) -> CmsResult<CmsEntry> {
        ctx.require_permission("cms.entry.update")?;
        let entry = self.repository().replace_entry_body(ctx, command).await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "entry".to_string(),
                    aggregate_id: entry.id,
                    event_type: CmsEventType::EntryUpdated,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "entry_id": entry.id,
                        "change": "body",
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        Ok(entry)
    }

    pub async fn replace_entry_fields(
        &self,
        ctx: &CmsRequestContext,
        command: EntryFieldsCommand,
    ) -> CmsResult<CmsEntry> {
        ctx.require_permission("cms.entry.update")?;
        let entry = self.repository().replace_entry_fields(ctx, command).await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "entry".to_string(),
                    aggregate_id: entry.id,
                    event_type: CmsEventType::EntryUpdated,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "entry_id": entry.id,
                        "change": "fields",
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        Ok(entry)
    }

    pub async fn list_entry_media(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryMediaQuery,
    ) -> CmsResult<CmsMediaRefPage> {
        ctx.require_permission("cms.entry.read")?;
        self.repository().list_entry_media(ctx, query).await
    }

    pub async fn attach_entry_media(
        &self,
        ctx: &CmsRequestContext,
        command: EntryMediaCommand,
    ) -> CmsResult<CmsMediaRef> {
        ctx.require_permission("cms.entry.update")?;
        self.repository().attach_entry_media(ctx, command).await
    }

    pub async fn delete_entry_media(
        &self,
        ctx: &CmsRequestContext,
        media_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.entry.update")?;
        self.repository().delete_entry_media(ctx, media_id).await
    }

    pub async fn replace_entry_terms(
        &self,
        ctx: &CmsRequestContext,
        command: ReplaceEntryTermsCommand,
    ) -> CmsResult<CmsEntry> {
        ctx.require_permission("cms.entry.update")?;
        self.repository().replace_entry_terms(ctx, command).await
    }

    pub async fn list_entry_versions(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryVersionsQuery,
    ) -> CmsResult<CmsEntryVersionPage> {
        ctx.require_permission("cms.entry.read")?;
        self.repository().list_entry_versions(ctx, query).await
    }

    pub async fn publish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        ctx.require_permission("cms.entry.publish")?;
        let snapshot = self
            .repository()
            .publish_entry(ctx, command.clone())
            .await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "entry".to_string(),
                    aggregate_id: command.owner_id,
                    event_type: CmsEventType::EntryPublished,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "entry_id": command.owner_id,
                        "snapshot_id": snapshot.id,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        if let Some(port) = self.search_sync_port() {
            let _ = port
                .request_search_sync(ctx, "entry", command.owner_id)
                .await;
        }
        if let Some(port) = self.cache_invalidation_port() {
            let _ = port
                .request_cache_invalidation(ctx, &format!("entry:{}", command.owner_id))
                .await;
        }
        if let Some(port) = self.sitemap_projection_port() {
            let _ = port.request_sitemap_projection(ctx, 0).await;
        }
        Ok(snapshot)
    }

    pub async fn unpublish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        ctx.require_permission("cms.entry.publish")?;
        let snapshot = self
            .repository()
            .unpublish_entry(ctx, command.clone())
            .await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "entry".to_string(),
                    aggregate_id: command.owner_id,
                    event_type: CmsEventType::EntryUnpublished,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "entry_id": command.owner_id,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        if let Some(port) = self.search_sync_port() {
            let _ = port
                .request_search_sync(ctx, "entry", command.owner_id)
                .await;
        }
        if let Some(port) = self.cache_invalidation_port() {
            let _ = port
                .request_cache_invalidation(ctx, &format!("entry:{}", command.owner_id))
                .await;
        }
        Ok(snapshot)
    }

    pub async fn rollback_entry(
        &self,
        ctx: &CmsRequestContext,
        command: RollbackCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        ctx.require_permission("cms.entry.rollback")?;
        let snapshot = self
            .repository()
            .rollback_entry(ctx, command.clone())
            .await?;
        let _ = self
            .event_publisher()
            .enqueue(
                ctx,
                CmsOutboxEventDraft {
                    aggregate_type: "entry".to_string(),
                    aggregate_id: command.owner_id,
                    event_type: CmsEventType::EntryRolledBack,
                    payload_json: serde_json::to_string(&serde_json::json!({
                        "entry_id": command.owner_id,
                        "target_version_id": command.target_version_id,
                    }))
                    .unwrap_or_default(),
                },
            )
            .await;
        Ok(snapshot)
    }

    pub async fn schedule_entry(
        &self,
        ctx: &CmsRequestContext,
        command: ScheduleCommand,
    ) -> CmsResult<CmsEntry> {
        ctx.require_permission("cms.entry.publish")?;
        let entry = self
            .repository()
            .schedule_entry(ctx, command.clone())
            .await?;
        if let Some(port) = self.scheduler_port() {
            if let Some(ref run_at) = command.scheduled_publish_at {
                let _ = port
                    .schedule_publish_job(ctx, "entry", command.entry_id, run_at)
                    .await;
            }
            if let Some(ref run_at) = command.scheduled_unpublish_at {
                let _ = port
                    .schedule_unpublish_job(ctx, "entry", command.entry_id, run_at)
                    .await;
            }
        }
        Ok(entry)
    }
}
