use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::{CmsError, CmsResult};
use crate::service::CmsService;

fn validate_code(code: &str) -> CmsResult<()> {
    if code.is_empty() || code.len() > 63 {
        return Err(CmsError::validation("code must be 1-63 characters"));
    }
    let first = code.as_bytes()[0];
    if !first.is_ascii_lowercase() {
        return Err(CmsError::validation(
            "code must start with a lowercase letter",
        ));
    }
    for ch in code.chars() {
        if !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '_' {
            return Err(CmsError::validation(
                "code must contain only lowercase letters, digits, and underscores",
            ));
        }
    }
    Ok(())
}

fn validate_name(name: &str) -> CmsResult<()> {
    if name.is_empty() || name.len() > 200 {
        return Err(CmsError::validation("name must be 1-200 characters"));
    }
    Ok(())
}

fn validate_content_kind(kind: &str) -> CmsResult<()> {
    match kind {
        "entry" | "page" | "block" | "fragment" => Ok(()),
        _ => Err(CmsError::validation(
            "content_kind must be one of: entry, page, block, fragment",
        )),
    }
}

fn validate_field_kind(kind: &str) -> CmsResult<()> {
    match kind {
        "text" | "richtext" | "integer" | "decimal" | "boolean" | "datetime" | "enum" | "media" | "reference" | "json" | "tags" => Ok(()),
        _ => Err(CmsError::validation("field_kind must be one of: text, richtext, integer, decimal, boolean, datetime, enum, media, reference, json, tags")),
    }
}

impl CmsService {
    pub async fn list_content_types(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsContentTypePage> {
        ctx.require_permission("cms.content_type.read")?;
        self.repository().list_content_types(ctx, query).await
    }

    pub async fn create_content_type(
        &self,
        ctx: &CmsRequestContext,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType> {
        ctx.require_permission("cms.content_type.manage")?;

        if let Some(ref code) = command.code {
            validate_code(code)?;
        }
        if let Some(ref name) = command.name {
            validate_name(name)?;
        }
        if let Some(ref kind) = command.content_kind {
            validate_content_kind(kind)?;
        }

        self.repository().create_content_type(ctx, command).await
    }

    pub async fn retrieve_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CmsContentType> {
        ctx.require_permission("cms.content_type.read")?;
        self.repository()
            .retrieve_content_type(ctx, content_type_id)
            .await
    }

    pub async fn update_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
        command: ContentTypeCommand,
    ) -> CmsResult<CmsContentType> {
        ctx.require_permission("cms.content_type.manage")?;

        if let Some(ref code) = command.code {
            validate_code(code)?;
        }
        if let Some(ref name) = command.name {
            validate_name(name)?;
        }
        if let Some(ref kind) = command.content_kind {
            validate_content_kind(kind)?;
        }

        self.repository()
            .update_content_type(ctx, content_type_id, command)
            .await
    }

    pub async fn delete_content_type(
        &self,
        ctx: &CmsRequestContext,
        content_type_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.content_type.manage")?;
        self.repository()
            .delete_content_type(ctx, content_type_id)
            .await
    }

    pub async fn list_content_fields(
        &self,
        ctx: &CmsRequestContext,
        query: ListContentFieldsQuery,
    ) -> CmsResult<CmsContentFieldPage> {
        ctx.require_permission("cms.content_type.read")?;
        self.repository().list_content_fields(ctx, query).await
    }

    pub async fn create_content_field(
        &self,
        ctx: &CmsRequestContext,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField> {
        ctx.require_permission("cms.content_type.manage")?;

        if let Some(ref code) = command.code {
            validate_code(code)?;
        }
        if let Some(ref name) = command.name {
            validate_name(name)?;
        }
        if let Some(ref kind) = command.field_kind {
            validate_field_kind(kind)?;
        }

        self.repository().create_content_field(ctx, command).await
    }

    pub async fn update_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
        command: ContentFieldCommand,
    ) -> CmsResult<CmsContentField> {
        ctx.require_permission("cms.content_type.manage")?;

        if let Some(ref code) = command.code {
            validate_code(code)?;
        }
        if let Some(ref name) = command.name {
            validate_name(name)?;
        }
        if let Some(ref kind) = command.field_kind {
            validate_field_kind(kind)?;
        }

        self.repository()
            .update_content_field(ctx, field_id, command)
            .await
    }

    pub async fn delete_content_field(
        &self,
        ctx: &CmsRequestContext,
        field_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.content_type.manage")?;
        self.repository().delete_content_field(ctx, field_id).await
    }
}
