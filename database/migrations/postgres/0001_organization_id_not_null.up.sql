-- sdkwork:migration
-- id: 0001_organization_id_not_null
-- engine: postgres
-- module: sdkwork-cms
-- purpose: Enforce organization_id NOT NULL DEFAULT on all tables in the
--   consolidated baseline. NULL rows (pre-standard data anomalies) are
--   backfilled with the platform sentinel before NOT NULL is set, and
--   NOT NULL columns without an explicit default receive the sentinel
--   default, keeping existing deployments consistent with fresh baseline
--   installs.
-- reversible: false
-- rollback: forward-fix (sentinel backfill is the canonical fix; NULL
--   organization rows are data anomalies)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

UPDATE cms_site SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_site ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_site ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_channel SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_channel ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_channel ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_content_type SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_content_type ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_content_type ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_content_field SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_content_field ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_content_field ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_taxonomy SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_taxonomy ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_taxonomy ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_taxonomy_term SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_taxonomy_term ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_taxonomy_term ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_entry SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_entry ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_entry ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_entry_body SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_entry_body ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_entry_body ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_entry_field_value SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_entry_field_value ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_entry_field_value ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_entry_version SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_entry_version ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_entry_version ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_entry_media SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_entry_media ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_entry_media ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_entry_term SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_entry_term ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_entry_term ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_page SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_page ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_page ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_page_block SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_page_block ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_page_block ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_publish_snapshot SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_publish_snapshot ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_publish_snapshot ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_feed SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_feed ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_feed ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_feed_rule SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_feed_rule ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_feed_rule ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_feed_item SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_feed_item ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_feed_item ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_feed_snapshot SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_feed_snapshot ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_feed_snapshot ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_audit_log SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_audit_log ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_audit_log ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_outbox_event SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_outbox_event ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_outbox_event ALTER COLUMN organization_id SET NOT NULL;

UPDATE cms_idempotency_key SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE cms_idempotency_key ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE cms_idempotency_key ALTER COLUMN organization_id SET NOT NULL;

COMMIT;
