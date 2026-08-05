use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn create_entry(
        &self,
        ctx: &CmsRequestContext,
        command: EntryCommand,
    ) -> CmsResult<CmsEntry> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();

        let row = sqlx::query_as::<_, crate::db::rows::CmsEntryRow>(
            "INSERT INTO cms_entry (id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, author_user_id, owner_user_id, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 0, 0, $12, $12, $13, $13, $12, $12, 0) 
             RETURNING id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, published_at, version"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.site_id)
        .bind(command.content_type_id)
        .bind(command.channel_id)
        .bind(&command.locale)
        .bind(&command.title)
        .bind(&command.slug)
        .bind(&command.summary)
        .bind(ctx.user_id)
        .bind(&now)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_entry_site_slug_locale") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "entry slug '{}' already exists in site/channel/locale",
                    command.slug
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(crate::mapper::row_mapper::map_entry_row(row))
    }

    pub async fn update_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
        command: EntryCommand,
    ) -> CmsResult<CmsEntry> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row = sqlx::query_as::<_, crate::db::rows::CmsEntryRow>(
            "UPDATE cms_entry SET 
                channel_id = COALESCE($4, channel_id),
                locale = COALESCE($5, locale),
                title = COALESCE($6, title),
                slug = COALESCE($7, slug),
                summary = COALESCE($8, summary),
                updated_at = $9,
                updated_by = $10,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, published_at, version"
        )
        .bind(ctx.tenant_id)
        .bind(entry_id)
        .bind(version)
        .bind(command.channel_id)
        .bind(if command.locale.is_empty() { None } else { Some(&command.locale) })
        .bind(if command.title.is_empty() { None } else { Some(&command.title) })
        .bind(if command.slug.is_empty() { None } else { Some(&command.slug) })
        .bind(&command.summary)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "entry",
            resource_id: entry_id,
            expected_version: version,
        })?;

        Ok(crate::mapper::row_mapper::map_entry_row(row))
    }

    pub async fn delete_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_entry SET entry_status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(entry_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("entry"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(entry_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn retrieve_entry(
        &self,
        ctx: &CmsRequestContext,
        entry_id: CmsId,
    ) -> CmsResult<CmsEntry> {
        let row = sqlx::query_as::<_, crate::db::rows::CmsEntryRow>(
            "SELECT id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, published_at, version 
             FROM cms_entry WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(entry_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::not_found("entry"))?;

        Ok(crate::mapper::row_mapper::map_entry_row(row))
    }

    pub async fn list_entries(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntriesQuery,
    ) -> CmsResult<CmsEntryPage> {
        let limit = query.limit.min(100) as i64;

        let mut sql = String::from(
            "SELECT id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, published_at, version 
             FROM cms_entry WHERE tenant_id = $1 AND deleted_at IS NULL"
        );
        let mut bind_idx = 2;

        if query.site_id.is_some() {
            sql.push_str(&format!(" AND site_id = ${}", bind_idx));
            bind_idx += 1;
        }
        if query.content_type_id.is_some() {
            sql.push_str(&format!(" AND content_type_id = ${}", bind_idx));
            bind_idx += 1;
        }
        if query.channel_id.is_some() {
            sql.push_str(&format!(" AND channel_id = ${}", bind_idx));
            bind_idx += 1;
        }
        if query.entry_status.is_some() {
            sql.push_str(&format!(" AND entry_status = ${}", bind_idx));
            bind_idx += 1;
        }
        if query.publication_status.is_some() {
            sql.push_str(&format!(" AND publication_status = ${}", bind_idx));
            bind_idx += 1;
        }

        sql.push_str(&format!(" ORDER BY updated_at DESC, id DESC LIMIT ${}", bind_idx));

        let mut q = sqlx::query_as::<_, crate::db::rows::CmsEntryRow>(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(ctx.tenant_id);

        if let Some(site_id) = query.site_id {
            q = q.bind(site_id);
        }
        if let Some(content_type_id) = query.content_type_id {
            q = q.bind(content_type_id);
        }
        if let Some(channel_id) = query.channel_id {
            q = q.bind(channel_id);
        }
        if let Some(entry_status) = query.entry_status {
            q = q.bind(entry_status);
        }
        if let Some(publication_status) = query.publication_status {
            q = q.bind(publication_status);
        }
        q = q.bind(limit);

        let rows = q
            .fetch_all(self.pool())
            .await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows.into_iter().map(crate::mapper::row_mapper::map_entry_row).collect();
        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn replace_entry_body(
        &self,
        ctx: &CmsRequestContext,
        command: EntryBodyCommand,
    ) -> CmsResult<CmsEntry> {
        let now = self.current_timestamp();
        let body_id = self.generate_id();
        let body_uuid = self.generate_uuid();

        sqlx::query(
            "INSERT INTO cms_entry_body (id, uuid, tenant_id, organization_id, site_id, entry_id, locale, body_format, body_text, body_html, block_tree_json, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, (SELECT site_id FROM cms_entry WHERE id = $5 AND tenant_id = $3), $5, $6, $7, $8, $9, $10, $11, $11, $12, $12, 0)
             ON CONFLICT (entry_id, locale) DO UPDATE SET 
                body_format = EXCLUDED.body_format,
                body_text = EXCLUDED.body_text,
                body_html = EXCLUDED.body_html,
                block_tree_json = EXCLUDED.block_tree_json,
                updated_at = EXCLUDED.updated_at,
                updated_by = EXCLUDED.updated_by,
                version = cms_entry_body.version + 1"
        )
        .bind(body_id)
        .bind(&body_uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.entry_id)
        .bind(&command.locale)
        .bind(&command.body_format)
        .bind(&command.body_text)
        .bind(&command.body_html)
        .bind(&command.block_tree_json)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        self.retrieve_entry(ctx, command.entry_id).await
    }

    pub async fn replace_entry_fields(
        &self,
        ctx: &CmsRequestContext,
        command: EntryFieldsCommand,
    ) -> CmsResult<CmsEntry> {
        let now = self.current_timestamp();

        let mut tx = self.pool().begin().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        // Delete existing field values for this entry and locale
        sqlx::query(
            "DELETE FROM cms_entry_field_value WHERE tenant_id = $1 AND entry_id = $2 AND locale = $3"
        )
        .bind(ctx.tenant_id)
        .bind(command.entry_id)
        .bind(&command.locale)
        .execute(&mut *tx)
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        // Parse and insert new field values
        let fields: serde_json::Value = serde_json::from_str(&command.fields_json)
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::validation(
                format!("invalid fields_json: {}", e)
            ))?;

        if let Some(fields_obj) = fields.as_object() {
            for (field_code, value) in fields_obj {
                let row_id = self.generate_id();
                let row_uuid = self.generate_uuid();

                // Look up the field_id from cms_content_field by code
                let field_id: Option<i64> = sqlx::query_scalar(
                    "SELECT id FROM cms_content_field WHERE code = $1 AND content_type_id = (SELECT content_type_id FROM cms_entry WHERE id = $2 AND tenant_id = $3) AND tenant_id = $3 AND deleted_at IS NULL"
                )
                .bind(field_code)
                .bind(command.entry_id)
                .bind(ctx.tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

                let field_id = match field_id {
                    Some(id) => id,
                    None => continue, // Skip unknown field codes
                };

                let (value_text, value_number, value_boolean, value_datetime, value_json): (Option<String>, Option<f64>, Option<bool>, Option<String>, String) = match value {
                    serde_json::Value::String(s) => (Some(s.clone()), None, None, None, serde_json::Value::Null.to_string()),
                    serde_json::Value::Number(n) => (None, n.as_f64(), None, None, serde_json::Value::Null.to_string()),
                    serde_json::Value::Bool(b) => (None, None, Some(*b), None, serde_json::Value::Null.to_string()),
                    serde_json::Value::Null => (None, None, None, None, serde_json::Value::Null.to_string()),
                    other => (None, None, None, None, other.to_string()),
                };

                sqlx::query(
                    "INSERT INTO cms_entry_field_value (id, uuid, tenant_id, organization_id, site_id, entry_id, field_id, locale, value_text, value_number, value_boolean, value_datetime, value_json, created_at, updated_at) 
                     VALUES ($1, $2, $3, $4, (SELECT site_id FROM cms_entry WHERE id = $5 AND tenant_id = $3), $5, $6, $7, $8, $9, $10, $11, $12, $13, $13)"
                )
                .bind(row_id)
                .bind(&row_uuid)
                .bind(ctx.tenant_id)
                .bind(ctx.organization_id)
                .bind(command.entry_id)
                .bind(field_id)
                .bind(&command.locale)
                .bind(&value_text)
                .bind(value_number)
                .bind(value_boolean)
                .bind(&value_datetime)
                .bind(&value_json)
                .bind(&now)
                .execute(&mut *tx)
                .await
                .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;
            }
        }

        tx.commit().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        self.retrieve_entry(ctx, command.entry_id).await
    }

    pub async fn list_entry_media(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryMediaQuery,
    ) -> CmsResult<CmsMediaRefPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, String, Option<String>, Option<String>, Option<String>, Option<String>, String)> = sqlx::query_as(
            "SELECT id, media_role, drive_space_id, drive_node_id, drive_uri, media_resource_id, media_snapshot_json 
             FROM cms_entry_media WHERE tenant_id = $1 AND entry_id = $2 AND status = 1 ORDER BY sort_order, id LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.entry_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, role, drive_space_id, drive_node_id, drive_uri, media_resource_id, media_snapshot_json)| {
                CmsMediaRef {
                    id,
                    role,
                    drive_space_id,
                    drive_node_id,
                    drive_uri,
                    media_resource_id,
                    media_snapshot_json,
                }
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn attach_entry_media(
        &self,
        ctx: &CmsRequestContext,
        command: EntryMediaCommand,
    ) -> CmsResult<CmsMediaRef> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();

        let row: (i64, String, Option<String>, Option<String>, Option<String>, Option<String>, String) = sqlx::query_as(
            "INSERT INTO cms_entry_media (id, uuid, tenant_id, organization_id, site_id, entry_id, field_id, media_role, drive_space_id, drive_node_id, drive_uri, media_resource_id, media_snapshot_json, alt_text, caption, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, (SELECT site_id FROM cms_entry WHERE id = $5 AND tenant_id = $3), $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, 1, $15, $15, $16, $16, 0) 
             RETURNING id, media_role, drive_space_id, drive_node_id, drive_uri, media_resource_id, media_snapshot_json"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.entry_id)
        .bind(command.field_id)
        .bind(&command.media_role)
        .bind(&command.drive_space_id)
        .bind(&command.drive_node_id)
        .bind(&command.drive_uri)
        .bind(&command.media_resource_id)
        .bind(&command.media_snapshot_json)
        .bind(&command.alt_text)
        .bind(&command.caption)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(CmsMediaRef {
            id: row.0,
            role: row.1,
            drive_space_id: row.2,
            drive_node_id: row.3,
            drive_uri: row.4,
            media_resource_id: row.5,
            media_snapshot_json: row.6,
        })
    }

    pub async fn delete_entry_media(
        &self,
        ctx: &CmsRequestContext,
        media_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_entry_media SET status = 9, updated_at = $3 WHERE tenant_id = $1 AND id = $2 AND status = 1"
        )
        .bind(ctx.tenant_id)
        .bind(media_id)
        .bind(&now)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("entry media"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(media_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn replace_entry_terms(
        &self,
        ctx: &CmsRequestContext,
        command: ReplaceEntryTermsCommand,
    ) -> CmsResult<CmsEntry> {
        let now = self.current_timestamp();

        let mut tx = self.pool().begin().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        sqlx::query("DELETE FROM cms_entry_term WHERE tenant_id = $1 AND entry_id = $2")
            .bind(ctx.tenant_id)
            .bind(command.entry_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        for term_id in &command.term_ids {
            let rel_id = self.generate_id();
            let rel_uuid = self.generate_uuid();
            sqlx::query(
                "INSERT INTO cms_entry_term (id, uuid, tenant_id, organization_id, site_id, entry_id, taxonomy_id, term_id, created_at, created_by) 
                 VALUES ($1, $2, $3, $4, (SELECT site_id FROM cms_entry WHERE id = $5 AND tenant_id = $3), $5, (SELECT taxonomy_id FROM cms_taxonomy_term WHERE id = $6 AND tenant_id = $3), $6, $7, $8)"
            )
            .bind(rel_id)
            .bind(&rel_uuid)
            .bind(ctx.tenant_id)
            .bind(ctx.organization_id)
            .bind(command.entry_id)
            .bind(term_id)
            .bind(&now)
            .bind(ctx.user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;
        }

        tx.commit().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        self.retrieve_entry(ctx, command.entry_id).await
    }

    pub async fn list_entry_versions(
        &self,
        ctx: &CmsRequestContext,
        query: ListEntryVersionsQuery,
    ) -> CmsResult<CmsEntryVersionPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, i64, String, Option<String>)> = sqlx::query_as(
            "SELECT id, entry_id, version_no, version_kind, checksum 
             FROM cms_entry_version WHERE tenant_id = $1 AND entry_id = $2 ORDER BY version_no DESC, id DESC LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.entry_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, entry_id, version_no, version_kind, checksum)| CmsEntryVersion {
                id,
                entry_id,
                version_no,
                version_kind,
                checksum,
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn publish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        let now = self.current_timestamp();
        let snapshot_id = self.generate_id();
        let snapshot_uuid = self.generate_uuid();

        let entry = self.retrieve_entry(ctx, command.owner_id).await?;

        let mut tx = self.pool().begin().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let row = sqlx::query_as::<_, crate::db::rows::CmsPublishSnapshotRow>(
            "INSERT INTO cms_publish_snapshot (id, uuid, tenant_id, organization_id, site_id, owner_type, owner_id, channel_id, locale, snapshot_payload_json, status, published_at, published_by, created_at) 
             VALUES ($1, $2, $3, $4, $5, 'entry', $6, $7, $8, $9, 1, $10, $11, $10)
             RETURNING id, tenant_id, site_id, owner_type, owner_id, snapshot_payload_json, status, published_at"
        )
        .bind(snapshot_id)
        .bind(&snapshot_uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(entry.site_id)
        .bind(command.owner_id)
        .bind(command.channel_id)
        .bind(command.locale.as_deref().unwrap_or("default"))
        .bind("{}")
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        sqlx::query(
            "UPDATE cms_entry SET entry_status = 30, publication_status = 20, published_at = $3, published_version_id = current_version_id, updated_at = $3, updated_by = $4, version = version + 1 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(command.owner_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        tx.commit().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(crate::mapper::row_mapper::map_publish_snapshot_row(row))
    }

    pub async fn unpublish_entry(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        let now = self.current_timestamp();
        let snapshot_id = self.generate_id();
        let snapshot_uuid = self.generate_uuid();

        let entry = self.retrieve_entry(ctx, command.owner_id).await?;

        let mut tx = self.pool().begin().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let row = sqlx::query_as::<_, crate::db::rows::CmsPublishSnapshotRow>(
            "INSERT INTO cms_publish_snapshot (id, uuid, tenant_id, organization_id, site_id, owner_type, owner_id, channel_id, locale, snapshot_payload_json, status, published_at, published_by, created_at) 
             VALUES ($1, $2, $3, $4, $5, 'entry', $6, $7, $8, '{}', 0, $9, $10, $9)
             RETURNING id, tenant_id, site_id, owner_type, owner_id, snapshot_payload_json, status, published_at"
        )
        .bind(snapshot_id)
        .bind(&snapshot_uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(entry.site_id)
        .bind(command.owner_id)
        .bind(command.channel_id)
        .bind(command.locale.as_deref().unwrap_or("default"))
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        sqlx::query(
            "UPDATE cms_entry SET publication_status = 30, updated_at = $3, updated_by = $4, version = version + 1 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(command.owner_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        tx.commit().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(crate::mapper::row_mapper::map_publish_snapshot_row(row))
    }

    pub async fn rollback_entry(
        &self,
        ctx: &CmsRequestContext,
        command: RollbackCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        let now = self.current_timestamp();
        let snapshot_id = self.generate_id();
        let snapshot_uuid = self.generate_uuid();

        let entry = self.retrieve_entry(ctx, command.owner_id).await?;

        let mut tx = self.pool().begin().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let row = sqlx::query_as::<_, crate::db::rows::CmsPublishSnapshotRow>(
            "INSERT INTO cms_publish_snapshot (id, uuid, tenant_id, organization_id, site_id, owner_type, owner_id, snapshot_payload_json, status, published_at, published_by, created_at) 
             VALUES ($1, $2, $3, $4, $5, 'entry', $6, '{}', 1, $7, $8, $7)
             RETURNING id, tenant_id, site_id, owner_type, owner_id, snapshot_payload_json, status, published_at"
        )
        .bind(snapshot_id)
        .bind(&snapshot_uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(entry.site_id)
        .bind(command.owner_id)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        sqlx::query(
            "UPDATE cms_entry SET publication_status = 40, updated_at = $3, updated_by = $4, version = version + 1 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(command.owner_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        tx.commit().await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(crate::mapper::row_mapper::map_publish_snapshot_row(row))
    }

    pub async fn schedule_entry(
        &self,
        ctx: &CmsRequestContext,
        command: ScheduleCommand,
    ) -> CmsResult<CmsEntry> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row = sqlx::query_as::<_, crate::db::rows::CmsEntryRow>(
            "UPDATE cms_entry SET 
                scheduled_publish_at = $4,
                scheduled_unpublish_at = $5,
                publication_status = 10,
                updated_at = $6,
                updated_by = $7,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, published_at, version"
        )
        .bind(ctx.tenant_id)
        .bind(command.entry_id)
        .bind(version)
        .bind(&command.scheduled_publish_at)
        .bind(&command.scheduled_unpublish_at)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "entry",
            resource_id: command.entry_id,
            expected_version: version,
        })?;

        Ok(crate::mapper::row_mapper::map_entry_row(row))
    }
}
