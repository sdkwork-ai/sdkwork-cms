use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::CmsResult;
use crate::service::CmsService;

impl CmsService {
    pub async fn list_taxonomies(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsTaxonomyPage> {
        ctx.require_permission("cms.taxonomy.read")?;
        self.repository().list_taxonomies(ctx, query).await
    }

    pub async fn create_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy> {
        ctx.require_permission("cms.taxonomy.manage")?;
        self.repository().create_taxonomy(ctx, command).await
    }

    pub async fn update_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy> {
        ctx.require_permission("cms.taxonomy.manage")?;
        self.repository()
            .update_taxonomy(ctx, taxonomy_id, command)
            .await
    }

    pub async fn delete_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.taxonomy.manage")?;
        self.repository().delete_taxonomy(ctx, taxonomy_id).await
    }

    pub async fn list_taxonomy_terms(
        &self,
        ctx: &CmsRequestContext,
        query: ListTaxonomyTermsQuery,
    ) -> CmsResult<CmsTaxonomyTermPage> {
        ctx.require_permission("cms.taxonomy.read")?;
        self.repository().list_taxonomy_terms(ctx, query).await
    }

    pub async fn create_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm> {
        ctx.require_permission("cms.taxonomy.manage")?;
        self.repository().create_taxonomy_term(ctx, command).await
    }

    pub async fn update_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm> {
        ctx.require_permission("cms.taxonomy.manage")?;
        self.repository()
            .update_taxonomy_term(ctx, term_id, command)
            .await
    }

    pub async fn delete_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.taxonomy.manage")?;
        self.repository().delete_taxonomy_term(ctx, term_id).await
    }
}
