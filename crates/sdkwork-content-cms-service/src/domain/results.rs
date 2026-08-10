use super::models::*;
use super::value_objects::{CmsId, CmsPage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandResult {
    pub ok: bool,
    pub resource_id: Option<CmsId>,
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsDeliveryBootstrap {
    pub site: CmsSite,
    pub channels: Vec<CmsChannel>,
}

pub type CmsSitePage = CmsPage<CmsSite>;
pub type CmsChannelPage = CmsPage<CmsChannel>;
pub type CmsContentTypePage = CmsPage<CmsContentType>;
pub type CmsContentFieldPage = CmsPage<CmsContentField>;
pub type CmsTaxonomyPage = CmsPage<CmsTaxonomy>;
pub type CmsTaxonomyTermPage = CmsPage<CmsTaxonomyTerm>;
pub type CmsEntryPage = CmsPage<CmsEntry>;
pub type CmsEntryVersionPage = CmsPage<CmsEntryVersion>;
pub type CmsMediaRefPage = CmsPage<CmsMediaRef>;
pub type CmsPagePage = CmsPage<CmsPageModel>;
pub type CmsFeedPage = CmsPage<CmsFeed>;
pub type CmsFeedRulePage = CmsPage<CmsFeedRule>;
pub type CmsFeedItemPage = CmsPage<CmsFeedItem>;
pub type CmsFavoritePage = CmsPage<CmsFavorite>;
pub type CmsAuditLogPage = CmsPage<CmsAuditLog>;
pub type CmsOutboxEventPage = CmsPage<CmsOutboxEvent>;
