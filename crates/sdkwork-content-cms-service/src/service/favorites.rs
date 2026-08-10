use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::{CmsError, CmsResult};
use crate::service::CmsService;

/// Favorite content shapes aligned with the favorites page filter tabs.
const FAVORITE_TYPES: &[&str] = &["link", "article", "image", "file", "voice", "chat"];

impl CmsService {
    pub async fn create_favorite(
        &self,
        ctx: &CmsRequestContext,
        command: FavoriteCommand,
    ) -> CmsResult<CmsFavorite> {
        if ctx.user_id <= 0 {
            return Err(CmsError::permission_denied("cms.favorites.manage"));
        }
        if !FAVORITE_TYPES.contains(&command.favorite_type.as_str()) {
            return Err(CmsError::validation(format!(
                "favorite_type must be one of {FAVORITE_TYPES:?}"
            )));
        }
        if command.target_type.trim().is_empty() {
            return Err(CmsError::validation("target_type is required"));
        }
        if command.target_type != "external_url" && command.target_id <= 0 {
            return Err(CmsError::validation(
                "target_id is required for non-external_url targets",
            ));
        }
        if command.target_type == "external_url"
            && command.target_url.as_deref().unwrap_or("").trim().is_empty()
        {
            return Err(CmsError::validation(
                "target_url is required for external_url targets",
            ));
        }
        if command.title.trim().is_empty() {
            return Err(CmsError::validation("title is required"));
        }
        self.repository().create_favorite(ctx, command).await
    }

    pub async fn list_favorites(
        &self,
        ctx: &CmsRequestContext,
        query: ListFavoritesQuery,
    ) -> CmsResult<CmsFavoritePage> {
        if ctx.user_id <= 0 {
            return Err(CmsError::permission_denied("cms.favorites.read"));
        }
        self.repository().list_favorites(ctx, query).await
    }

    pub async fn delete_favorite(
        &self,
        ctx: &CmsRequestContext,
        favorite_uuid: String,
    ) -> CmsResult<CommandResult> {
        if ctx.user_id <= 0 {
            return Err(CmsError::permission_denied("cms.favorites.manage"));
        }
        if favorite_uuid.trim().is_empty() {
            return Err(CmsError::validation("favorite_uuid is required"));
        }
        self.repository().delete_favorite(ctx, favorite_uuid).await
    }
}
