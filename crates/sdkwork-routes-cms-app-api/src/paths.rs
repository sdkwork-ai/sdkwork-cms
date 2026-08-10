pub const PREFIX: &str = "/app/v3/api/cms";

pub const BOOTSTRAP: &str = "/app/v3/api/cms/sites/{siteCode}/bootstrap";
pub const ENTRIES_RESOLVE: &str = "/app/v3/api/cms/sites/{siteCode}/entries:resolve";
pub const ENTRY_BY_ID: &str = "/app/v3/api/cms/entries/{entryId}";
pub const PAGES_RESOLVE: &str = "/app/v3/api/cms/sites/{siteCode}/pages:resolve";
pub const FEED_ITEMS: &str = "/app/v3/api/cms/sites/{siteCode}/feeds/{feedCode}/items";
pub const FAVORITES: &str = "/app/v3/api/cms/favorites";
pub const FAVORITE_BY_ID: &str = "/app/v3/api/cms/favorites/{favoriteId}";

pub const ALL_PATHS: &[&str] = &[
    BOOTSTRAP,
    ENTRIES_RESOLVE,
    ENTRY_BY_ID,
    PAGES_RESOLVE,
    FEED_ITEMS,
    FAVORITES,
    FAVORITE_BY_ID,
];
