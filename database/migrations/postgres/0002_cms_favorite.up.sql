-- sdkwork:migration
-- id: 0002_cms_favorite
-- engine: postgres
-- module: sdkwork-cms
-- purpose: Add cms_favorite table for the user favorites capability. Favorites
--   are user-owned polymorphic references to content objects across business
--   modules (im_message, drive_file, kb_article, cms_entry, external_url),
--   carrying a self-sufficient snapshot (title, summary, source display name,
--   media) so delivery lists never depend on the target domain. The unique
--   constraint on (tenant, organization, user, target_type, target_id) makes
--   repeated favoriting idempotent and refreshes the favorite to the top.
-- reversible: false
-- rollback: forward-fix (dropping the table loses user favorites)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

CREATE TABLE IF NOT EXISTS cms_favorite (
    id                   BIGINT PRIMARY KEY,
    uuid                 VARCHAR(64) NOT NULL UNIQUE,
    tenant_id            BIGINT NOT NULL,
    organization_id      BIGINT NOT NULL DEFAULT 0,
    user_id              BIGINT NOT NULL,
    favorite_type        VARCHAR(32) NOT NULL
                         CHECK (favorite_type IN
                           ('link','article','image','file','voice','chat')),
    target_type          VARCHAR(64) NOT NULL
                         CHECK (btrim(target_type) <> ''),
    target_id            BIGINT NOT NULL DEFAULT 0,
    target_uuid          VARCHAR(64),
    target_url           VARCHAR(2048),
    title                VARCHAR(512) NOT NULL DEFAULT '',
    summary              VARCHAR(2048) NOT NULL DEFAULT '',
    source_display_name  VARCHAR(256) NOT NULL DEFAULT '',
    media_json           JSONB NOT NULL DEFAULT '{}',
    favorited_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    version              BIGINT NOT NULL DEFAULT 0,
    deleted_at           TIMESTAMPTZ,
    deleted_by           BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT uk_cms_favorite_user_target UNIQUE
        (tenant_id, organization_id, user_id, target_type, target_id),
    CONSTRAINT chk_cms_favorite_user CHECK (user_id > 0),
    CONSTRAINT chk_cms_favorite_target CHECK (
        (target_type <> 'external_url' AND target_id > 0)
        OR (target_type = 'external_url' AND btrim(COALESCE(target_url, '')) <> '')
    )
);

CREATE INDEX IF NOT EXISTS idx_cms_favorite_user_ts ON cms_favorite
    (tenant_id, organization_id, user_id, favorited_at DESC);

CREATE INDEX IF NOT EXISTS idx_cms_favorite_user_type ON cms_favorite
    (tenant_id, organization_id, user_id, favorite_type, favorited_at DESC);

COMMIT;
