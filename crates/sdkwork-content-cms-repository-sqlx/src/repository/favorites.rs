use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

type CmsFavoriteRow = (
    i64,
    String,
    i64,
    String,
    String,
    i64,
    Option<String>,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
    i64,
);

fn map_favorite_row(row: CmsFavoriteRow) -> CmsFavorite {
    CmsFavorite {
        id: row.0,
        uuid: row.1,
        tenant_id: 0,
        organization_id: 0,
        user_id: row.2,
        favorite_type: row.3,
        target_type: row.4,
        target_id: row.5,
        target_uuid: row.6,
        target_url: row.7,
        title: row.8,
        summary: row.9,
        source_display_name: row.10,
        media_json: row.11,
        favorited_at: row.12,
        version: row.13,
    }
}

/// Keyset cursor "favorited_at|id" aligned with `ORDER BY favorited_at DESC, id DESC`.
fn encode_favorite_cursor(favorited_at: &str, id: i64) -> String {
    format!("{favorited_at}|{id}")
}

fn decode_favorite_cursor(cursor: &str) -> Option<(String, i64)> {
    let (time_part, id_part) = cursor.rsplit_once('|')?;
    let id = id_part.parse::<i64>().ok()?;
    Some((time_part.to_string(), id))
}

impl CmsSqlxRepository {
    pub async fn create_favorite(
        &self,
        ctx: &CmsRequestContext,
        command: FavoriteCommand,
    ) -> CmsResult<CmsFavorite> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let media_json = if command.media_json.trim().is_empty() {
            "{}"
        } else {
            command.media_json.as_str()
        };

        let row: CmsFavoriteRow = sqlx::query_as(
            "INSERT INTO cms_favorite (
                 id, uuid, tenant_id, organization_id, user_id, favorite_type,
                 target_type, target_id, target_uuid, target_url, title, summary,
                 source_display_name, media_json, favorited_at, created_at,
                 updated_at, created_by, updated_by, version
             )
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14::jsonb,
                     $15, $15, $15, $16, $16, 0)
             ON CONFLICT (tenant_id, organization_id, user_id, target_type, target_id) DO UPDATE
               SET favorite_type = EXCLUDED.favorite_type,
                   target_uuid = EXCLUDED.target_uuid,
                   target_url = EXCLUDED.target_url,
                   title = EXCLUDED.title,
                   summary = EXCLUDED.summary,
                   source_display_name = EXCLUDED.source_display_name,
                   media_json = EXCLUDED.media_json,
                   favorited_at = EXCLUDED.favorited_at,
                   updated_at = EXCLUDED.updated_at,
                   updated_by = EXCLUDED.updated_by,
                   version = cms_favorite.version + 1,
                   deleted_at = NULL
             RETURNING id, uuid, user_id, favorite_type, target_type, target_id,
                       target_uuid, target_url, title, summary, source_display_name,
                       media_json, favorited_at, version",
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(ctx.user_id)
        .bind(&command.favorite_type)
        .bind(&command.target_type)
        .bind(command.target_id)
        .bind(&command.target_uuid)
        .bind(&command.target_url)
        .bind(&command.title)
        .bind(&command.summary)
        .bind(&command.source_display_name)
        .bind(media_json)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(map_favorite_row(row))
    }

    pub async fn list_favorites(
        &self,
        ctx: &CmsRequestContext,
        query: ListFavoritesQuery,
    ) -> CmsResult<CmsFavoritePage> {
        let limit = query.limit.min(100) as i64;
        let favorite_type = query.favorite_type.filter(|v| !v.trim().is_empty());
        let search_query = query.search_query.filter(|v| !v.trim().is_empty());
        let (cursor_time, cursor_id) = query
            .cursor
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .and_then(decode_favorite_cursor)
            .map_or((None, None), |(time_part, id)| {
                (Some(time_part), Some(id))
            });

        let rows = sqlx::query_as::<_, CmsFavoriteRow>(
            "SELECT id, uuid, user_id, favorite_type, target_type, target_id,
                    target_uuid, target_url, title, summary, source_display_name,
                    media_json, favorited_at, version
             FROM cms_favorite
             WHERE tenant_id = $1 AND user_id = $2 AND deleted_at IS NULL
               AND ($3::text IS NULL OR favorite_type = $3)
               AND ($4::text IS NULL OR title ILIKE '%' || $4 || '%'
                    OR summary ILIKE '%' || $4 || '%'
                    OR source_display_name ILIKE '%' || $4 || '%')
               AND ($5::text IS NULL OR favorited_at < $5::timestamptz
                    OR (favorited_at = $5::timestamptz AND id < $6))
             ORDER BY favorited_at DESC, id DESC LIMIT $7",
        )
        .bind(ctx.tenant_id)
        .bind(ctx.user_id)
        .bind(favorite_type)
        .bind(search_query)
        .bind(cursor_time)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let has_more = rows.len() > limit as usize;
        let rows = rows.into_iter().take(limit as usize).collect::<Vec<_>>();
        let next_cursor = match (has_more, rows.last()) {
            (true, Some(last)) => Some(encode_favorite_cursor(&last.12, last.0)),
            _ => None,
        };

        Ok(CmsPage {
            items: rows.into_iter().map(map_favorite_row).collect(),
            next_cursor,
        })
    }

    pub async fn delete_favorite(
        &self,
        ctx: &CmsRequestContext,
        favorite_uuid: String,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_favorite
             SET deleted_at = $4, updated_at = $4, updated_by = $3, version = version + 1
             WHERE tenant_id = $1 AND uuid = $2 AND user_id = $3 AND deleted_at IS NULL",
        )
        .bind(ctx.tenant_id)
        .bind(&favorite_uuid)
        .bind(ctx.user_id)
        .bind(&now)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("favorite"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: None,
            request_id: Some(ctx.request_id.clone()),
        })
    }
}
