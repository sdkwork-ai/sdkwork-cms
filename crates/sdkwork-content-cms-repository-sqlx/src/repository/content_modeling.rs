use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn list_content_types(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsContentTypePage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, String, i64, String, String, String, i64, i32)> = sqlx::query_as(
            "SELECT id, uuid, site_id, code, name, content_kind, schema_version, status 
             FROM cms_content_type WHERE tenant_id = $1 AND site_id = $2 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.site_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, _uuid, site_id, code, name, content_kind, schema_version, status)| {
                CmsContentType {
                    id,
                    site_id,
                    code,
                    name,
                    content_kind,
                    schema_version,
                    status,
                }
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_content_type(
        &self,
        ctx: &CmsRequestContext,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let code = command.code.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("code is required")
        })?;
        let name = command.name.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("name is required")
        })?;
        let content_kind = command.content_kind.unwrap_or_else(|| "entry".to_string());

        let row: (i64, String, i64, String, String, String, i64, i32) = sqlx::query_as(
            "INSERT INTO cms_content_type (id, uuid, tenant_id, organization_id, site_id, code, name, content_kind, schema_version, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1, 1, $9, $9, $10, $10, 0) 
             RETURNING id, uuid, site_id, code, name, content_kind, schema_version, status"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.site_id)
        .bind(&code)
        .bind(&name)
        .bind(&content_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_content_type_site_code") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "content type code '{}' already exists in site",
                    code
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(CmsContentType {
            id: row.0,
            site_id: row.2,
            code: row.3,
            name: row.4,
            content_kind: row.5,
            schema_version: row.6,
            status: row.7,
        })
    }

    pub async fn retrieve_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CmsContentType> {
        let row: (i64, String, i64, String, String, String, i64, i32) = sqlx::query_as(
            "SELECT id, uuid, site_id, code, name, content_kind, schema_version, status 
             FROM cms_content_type WHERE tenant_id = $1 AND organization_id = $2 AND id = $3 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(content_type_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::not_found("content type"))?;

        Ok(CmsContentType {
            id: row.0,
            site_id: row.2,
            code: row.3,
            name: row.4,
            content_kind: row.5,
            schema_version: row.6,
            status: row.7,
        })
    }

    pub async fn update_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, String, i64, String, String, String, i64, i32) = sqlx::query_as(
            "UPDATE cms_content_type SET 
                code = COALESCE($5, code),
                name = COALESCE($6, name),
                content_kind = COALESCE($7, content_kind),
                updated_at = $8,
                updated_by = $9,
                version = version + 1
             WHERE tenant_id = $1 AND organization_id = $2 AND id = $3 AND version = $4 AND deleted_at IS NULL
             RETURNING id, uuid, site_id, code, name, content_kind, schema_version, status"
        )
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(content_type_id)
        .bind(version)
        .bind(&command.code)
        .bind(&command.name)
        .bind(&command.content_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "content_type",
            resource_id: content_type_id,
            expected_version: version,
        })?;

        Ok(CmsContentType {
            id: row.0,
            site_id: row.2,
            code: row.3,
            name: row.4,
            content_kind: row.5,
            schema_version: row.6,
            status: row.7,
        })
    }

    pub async fn delete_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_content_type SET status = 9, deleted_at = $4, deleted_by = $5 WHERE tenant_id = $1 AND organization_id = $2 AND id = $3 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(content_type_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("content type"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(content_type_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn list_content_fields(
        &self,
        ctx: &CmsRequestContext,
        query: ListContentFieldsQuery,
    ) -> CmsResult<CmsContentFieldPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, String, String, String, bool, bool, bool, bool)> = sqlx::query_as(
            "SELECT id, content_type_id, code, name, field_kind, required, searchable, filterable, sortable 
             FROM cms_content_field WHERE tenant_id = $1 AND content_type_id = $2 AND deleted_at IS NULL ORDER BY sort_order, id LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.content_type_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(
                |(id, content_type_id, code, name, field_kind, required, searchable, filterable, sortable)| {
                    CmsContentField {
                        id,
                        content_type_id,
                        code,
                        name,
                        field_kind,
                        required,
                        searchable,
                        filterable,
                        sortable,
                    }
                },
            )
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_content_field(
        &self,
        ctx: &CmsRequestContext,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let code = command.code.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("code is required")
        })?;
        let name = command.name.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("name is required")
        })?;
        let field_kind = command.field_kind.unwrap_or_else(|| "text".to_string());

        let row: (i64, i64, String, String, String, bool, bool, bool, bool) = sqlx::query_as(
            "INSERT INTO cms_content_field (id, uuid, tenant_id, organization_id, site_id, content_type_id, code, name, field_kind, required, searchable, filterable, sortable, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, (SELECT site_id FROM cms_content_type WHERE id = $5 AND tenant_id = $3), $5, $6, $7, $8, false, false, false, false, 1, $9, $9, $10, $10, 0) 
             RETURNING id, content_type_id, code, name, field_kind, required, searchable, filterable, sortable"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.content_type_id)
        .bind(&code)
        .bind(&name)
        .bind(&field_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_content_field_code") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "content field code '{}' already exists in content type",
                    code
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(CmsContentField {
            id: row.0,
            content_type_id: row.1,
            code: row.2,
            name: row.3,
            field_kind: row.4,
            required: row.5,
            searchable: row.6,
            filterable: row.7,
            sortable: row.8,
        })
    }

    pub async fn update_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, i64, String, String, String, bool, bool, bool, bool) = sqlx::query_as(
            "UPDATE cms_content_field SET 
                code = COALESCE($4, code),
                name = COALESCE($5, name),
                field_kind = COALESCE($6, field_kind),
                updated_at = $7,
                updated_by = $8,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, content_type_id, code, name, field_kind, required, searchable, filterable, sortable"
        )
        .bind(ctx.tenant_id)
        .bind(field_id)
        .bind(version)
        .bind(&command.code)
        .bind(&command.name)
        .bind(&command.field_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "content_field",
            resource_id: field_id,
            expected_version: version,
        })?;

        Ok(CmsContentField {
            id: row.0,
            content_type_id: row.1,
            code: row.2,
            name: row.3,
            field_kind: row.4,
            required: row.5,
            searchable: row.6,
            filterable: row.7,
            sortable: row.8,
        })
    }

    pub async fn delete_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_content_field SET status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(field_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("content field"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(field_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }
}
