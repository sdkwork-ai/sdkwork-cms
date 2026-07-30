> Migrated from `docs/plans/2026-06-13-cms-implementation-completion.md` on 2026-06-24.
> Owner: SDKWork maintainers

**Created**: 2026-06-13
**Status**: Ready for execution
**Scope**: All remaining TODOs in sdkwork-cms Rust crates

## Executive Summary

The sdkwork-cms codebase has a fully functional hexagonal architecture with all CRUD operations, permission checks, and handler wiring in place. The remaining work falls into 5 phases: dead code cleanup, row mapper consolidation, service-layer gaps, test coverage, and contract/registry synchronization.

## Phase 1: Dead Code Cleanup (Low Risk)

Remove old per-module stub files that duplicate what's already implemented inline in the main `handlers.rs` files.

### 1a. Open API — Remove dead handler/DTO/mapper stubs

The main `crates/sdkwork-routes-cms-open-api/src/handlers.rs` already contains full inline implementations for `list_entries`, `retrieve_entry`, `resolve_entry`, `resolve_page`, and `list_feed_items` with inline DTOs and mappers. The following files are dead code:

| File | Action | Reason |
|------|--------|--------|
| `src/handlers/entries.rs` | DELETE | 3 stub fns returning `not_implemented` |
| `src/handlers/pages.rs` | DELETE | 1 stub fn returning `not_implemented` |
| `src/handlers/feeds.rs` | DELETE | 1 stub fn returning `not_implemented` |
| `src/dto/request.rs` | DELETE | `CmsOpenApiRequestDto` with `not_implemented` methods |
| `src/dto/response.rs` | DELETE | `CmsOpenApiResponseDto` with `not_implemented` methods |
| `src/dto/mod.rs` | SIMPLIFY | Remove `pub mod request; pub mod response;` and re-exports, keep empty or delete |
| `src/mapper/request.rs` | DELETE | Empty stub `map_open_api_key_context()` |
| `src/mapper/response.rs` | DELETE | Empty stub `map_delivery_response()` |
| `src/mapper/problem.rs` | DELETE | Empty stub `map_problem_detail()` |
| `src/mapper/mod.rs` | DELETE | Only re-exports dead submodules |
| `src/lib.rs` | EDIT | Remove `pub mod dto;` and `pub mod mapper;` declarations |

### 1b. App API — Remove dead handler/DTO/mapper stubs

The main `crates/sdkwork-routes-cms-app-api/src/handlers.rs` already contains full inline implementations. The following are dead code:

| File | Action | Reason |
|------|--------|--------|
| `src/handlers/delivery.rs` | DELETE | 5 stub fns returning `not_implemented` |
| `src/dto/request.rs` | DELETE | `CmsAppApiRequestDto` with `not_implemented` methods |
| `src/dto/response.rs` | DELETE | `CmsAppApiResponseDto` with `not_implemented` methods |
| `src/dto/mod.rs` | SIMPLIFY | Remove dead submodules |
| `src/mapper/request.rs` | DELETE | Empty stub |
| `src/mapper/response.rs` | DELETE | Empty stub |
| `src/mapper/problem.rs` | DELETE | Empty stub |
| `src/mapper/mod.rs` | DELETE | Only re-exports dead submodules |
| `src/lib.rs` | EDIT | Remove `pub mod dto;` and `pub mod mapper;` declarations |

### 1c. Verification

- `cargo check --workspace` passes
- `node tests/static/cms-contract-boundary.test.mjs` — the handler skeleton tests will need updating (see Phase 5)

---

## Phase 2: Repository Row Mapper Consolidation (Medium Risk)

Currently only 3 of 22 tables have `FromRow` row structs. Many repository methods use inline tuple destructuring which is fragile and hard to maintain. Add missing row structs and migrate all repositories to use the centralized row mapper.

### 2a. Add missing row structs to `crates/sdkwork-content-cms-repository-sqlx/src/db/rows.rs`

Add `FromRow` structs for every table that a repository method queries:

| Row Struct | Table | Fields (from queries.rs and repository/*.rs) |
|------------|-------|----------------------------------------------|
| `CmsChannelRow` | cms_channel | id, uuid, site_id, code, name, channel_kind, status |
| `CmsContentTypeRow` | cms_content_type | id, uuid, site_id, code, name, content_kind, schema_version, status |
| `CmsContentFieldRow` | cms_content_field | id, content_type_id, code, name, field_kind, required, searchable, filterable, sortable |
| `CmsTaxonomyRow` | cms_taxonomy | id, site_id, code, name, taxonomy_kind, status |
| `CmsTaxonomyTermRow` | cms_taxonomy_term | id, taxonomy_id, parent_id, code, slug, name, path, status |
| `CmsEntryBodyRow` | cms_entry_body | (not directly queried as aggregate root — keep inline for upsert) |
| `CmsEntryVersionRow` | cms_entry_version | id, entry_id, version_no, version_kind, checksum |
| `CmsEntryMediaRow` | cms_entry_media | id, media_role, drive_space_id, drive_node_id, drive_uri, media_resource_id, media_snapshot_json |
| `CmsPageRow` | cms_page | id, site_id, channel_id, locale, path, title, publication_status, version |
| `CmsPageBlockRow` | cms_page_block | (keep inline for replace_page_blocks) |
| `CmsFeedRow` | cms_feed | id, site_id, channel_id, code, name, feed_kind, locale, status, version |
| `CmsFeedRuleRow` | cms_feed_rule | id, feed_id, rule_kind, condition_json, sort_json, enabled |
| `CmsFeedItemRow` | cms_feed_item | id, feed_id, entry_id, page_id, external_url, item_kind, pinned, sort_order |
| `CmsFeedSnapshotRow` | cms_feed_snapshot | id, feed_id, publish_snapshot_id, snapshot_version, item_count, items_json, status, published_at |
| `CmsAuditLogRow` | cms_audit_log | id, site_id, actor_user_id, action, resource_type, resource_id, before_json, after_json, created_at |
| `CmsOutboxEventRow` | cms_outbox_event | id, aggregate_type, aggregate_id, event_type, payload_json, status, attempt_count, next_attempt_at, created_at |

### 2b. Add mapper functions to `crates/sdkwork-content-cms-repository-sqlx/src/mapper/row_mapper.rs`

Add `map_*_row()` functions for each new row struct, following the existing pattern of `map_site_row` and `map_entry_row`. Each mapper converts the row struct to the corresponding domain model.

Key mapping logic:
- `CmsChannelRow` → `CmsChannel` (direct field copy)
- `CmsContentTypeRow` → `CmsContentType` (direct field copy)
- `CmsContentFieldRow` → `CmsContentField` (direct field copy)
- `CmsTaxonomyRow` → `CmsTaxonomy` (direct field copy)
- `CmsTaxonomyTermRow` → `CmsTaxonomyTerm` (direct field copy)
- `CmsEntryVersionRow` → `CmsEntryVersion` (direct field copy)
- `CmsEntryMediaRow` → `CmsMediaRef` (field rename: media_role → role)
- `CmsPageRow` → `CmsPageModel` (publication_status enum conversion, same as pages.rs:34-40)
- `CmsFeedRow` → `CmsFeed` (direct field copy)
- `CmsFeedRuleRow` → `CmsFeedRule` (direct field copy)
- `CmsFeedItemRow` → `CmsFeedItem` (direct field copy)
- `CmsFeedSnapshotRow` → `CmsFeedSnapshot` (direct field copy)
- `CmsAuditLogRow` → `CmsAuditLog` (direct field copy)
- `CmsOutboxEventRow` → `CmsOutboxEvent` (direct field copy)

### 2c. Migrate repository methods from tuple destructuring to row mapper

Update each repository module to use `query_as::<_, CmsChannelRow>()` + `map_channel_row()` instead of inline tuple destructuring. Files to update:

| File | Methods to migrate |
|------|-------------------|
| `repository/sites.rs` | `list_channels`, `create_channel`, `update_channel` |
| `repository/content_modeling.rs` | `list_content_types`, `create_content_type`, `retrieve_content_type`, `update_content_type`, `list_content_fields`, `create_content_field`, `update_content_field` |
| `repository/taxonomy.rs` | `list_taxonomies`, `create_taxonomy`, `update_taxonomy`, `list_taxonomy_terms`, `create_taxonomy_term`, `update_taxonomy_term` |
| `repository/entries.rs` | `list_entry_media`, `attach_entry_media`, `list_entry_versions` |
| `repository/pages.rs` | `list_pages`, `create_page`, `retrieve_page`, `update_page` |
| `repository/feeds.rs` | `list_feeds`, `create_feed`, `retrieve_feed`, `update_feed`, `list_feed_rules`, `create_feed_rule`, `update_feed_rule`, `list_feed_items`, `retrieve_feed_snapshot` |
| `repository/governance.rs` | `list_audit_logs`, `list_outbox_events` |

### 2d. Verification

- `cargo check --workspace` passes
- `cargo test --workspace` passes
- All existing smoke tests continue to pass

---

## Phase 3: Service Layer Gaps (Medium Risk)

### 3a. Schema validation for content types and fields

**File**: `crates/sdkwork-content-cms-service/src/service/content_modeling.rs`

Add validation logic in `create_content_type`, `update_content_type`, `create_content_field`, `update_content_field`:

**Content type validation** (`create_content_type`, `update_content_type`):
- `code` must match `^[a-z][a-z0-9_]{1,62}$` (lowercase alphanumeric + underscore)
- `name` must be 1-200 characters
- `content_kind` must be one of: `entry`, `page`, `block`, `fragment`
- On validation failure, return `CmsError::validation(msg)`

**Content field validation** (`create_content_field`, `update_content_field`):
- `code` must match `^[a-z][a-z0-9_]{1,62}$`
- `name` must be 1-200 characters
- `field_kind` must be one of: `text`, `richtext`, `integer`, `decimal`, `boolean`, `datetime`, `enum`, `media`, `reference`, `json`, `tags`
- On validation failure, return `CmsError::validation(msg)`

### 3b. Feed composition logic

**File**: `crates/sdkwork-content-cms-service/src/service/feeds.rs`

Add a `compose_feed` method that assembles feed items from multiple sources:

```rust
pub async fn compose_feed(
    &self,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
) -> CmsResult<CmsFeedSnapshot>
```

Logic:
1. Load the feed and its rules (`list_feed_rules`)
2. For each rule based on `rule_kind`:
   - `curated`: Load manually pinned items from `cms_feed_item`
   - `rule`: Execute the rule's `condition_json` filter against `list_entries` with appropriate filters
   - `hybrid`: Merge curated items first, then fill remaining slots with rule-based items
3. Deduplicate by entry_id/page_id
4. Respect `limit_count` per rule and total feed limit
5. Build a `CmsFeedSnapshot` with the composed `items_json`
6. Return the snapshot (do NOT persist — persistence is the caller's job via `publish_feed`)

### 3c. Verification

- `cargo check --workspace` passes
- `cargo test --workspace` passes

---

## Phase 4: Test Coverage (High Priority)

### 4a. Service unit tests

**File**: `crates/sdkwork-content-cms-service/tests/service_smoke.rs` (expand existing)

Create mock implementations of `CmsRepository`, `CmsIamAuthorizer`, and `CmsEventPublisher` traits for testing.

**New test file**: `crates/sdkwork-content-cms-service/tests/service_unit.rs`

Test categories:
1. **Permission enforcement**: Verify each service method calls `require_permission` with the correct permission code
2. **Entry lifecycle**: Test draft → review → approve → publish → unpublish → rollback → schedule flow
3. **Publish atomics**: Verify publish creates snapshot + updates entry status + enqueues outbox event
4. **Delivery queries**: Test `delivery_list_entries` filters by `publication_status = 20`
5. **Preview token**: Test `delivery_resolve_entry` with and without preview tokens
6. **Schema validation**: Test content type/field validation rejects invalid codes
7. **Feed composition**: Test curated, rule, and hybrid feed assembly
8. **Error mapping**: Verify `CmsError` variants map correctly

### 4b. Repository integration tests

**File**: `crates/sdkwork-content-cms-repository-sqlx/tests/repository_integration.rs`

Requires a test PostgreSQL profile. Use `sqlx::test` attribute or a test harness with `SDKWORK_DATABASE_TEST_POSTGRES_URL`.

Test categories:
1. **CRUD round-trips**: Create → retrieve → update → soft-delete for each aggregate
2. **Optimistic locking**: Verify version conflict returns `OptimisticLockConflict`
3. **Unique constraints**: Verify duplicate code/slug returns `Conflict` error
4. **Tenant isolation**: Verify queries filter by `tenant_id`
5. **Pagination**: Verify `limit` and `cursor` behavior
6. **Transactional writes**: Verify entry body + fields + media + terms + version are written atomically

### 4c. Route handler tests

**Files**: Each `tests/route_standard.rs` (expand existing)

Add handler-level tests using mock service:
1. **Request DTO deserialization**: Verify JSON bodies deserialize correctly
2. **Response DTO serialization**: Verify response shapes match OpenAPI spec
3. **Error responses**: Verify RFC 9457 problem detail format
4. **Permission errors**: Verify 403 responses for permission denied

### 4d. Verification

- `cargo test --workspace` passes
- All tests are deterministic and don't require external services (except repository integration tests)

---

## Phase 5: Contract and Registry Synchronization (Low Risk)

### 5a. Update contract boundary test

**File**: `tests/static/cms-contract-boundary.test.mjs`

The test at line 351 asserts that every service method has a `TODO:` comment. After implementing schema validation and feed composition, update:
- Remove the `TODO:` assertion for methods that are now fully implemented
- Or change the assertion to check for implementation evidence instead of TODO markers

The test at lines 506-511 asserts that every handler module file has `TODO:` markers. After Phase 1 dead code removal, update:
- Remove assertions for deleted open-api/app-api handler submodules
- Update the test to reflect the new file structure

### 5b. Update column registry TODOs

**File**: `docs/database/cms-v1-column-registry.json`

For each of the 22 tables, replace `implementationTodo: "TODO: Implement repository row mapping..."` with the actual implementation status:
- Tables with row mapper structs: mark as "Row mapping implemented via FromRow struct and map_*_row function"
- Tables using inline tuples: mark as "Row mapping implemented via inline query_as tuple destructuring"

### 5c. Update permission registry TODOs

**File**: `docs/security/cms-v1-permission-registry.json`

Replace `implementationTodo: "TODO: ..."` entries with actual implementation descriptions:
- All permissions are enforced via `ctx.require_permission()` in service methods
- Update each permission's TODO to describe the enforcement mechanism

### 5d. Update integration registry TODOs

**File**: `docs/integration/cms-v1-integration-registry.json`

Replace `implementationTodo: "TODO: ..."` entries with actual implementation descriptions:
- Events are persisted via `cms_outbox_event` in the same transaction
- Dependency ports are injected via `CmsService::with_*_port()` builder methods

### 5e. Update implementation-todos.md

**File**: `docs/implementation-todos.md`

Check off all completed items and update remaining items with current status.

### 5f. Verification

- `node tests/static/cms-contract-boundary.test.mjs` passes
- All registry files are internally consistent

---

## Execution Order

```
Phase 1 (Dead Code) ──→ Phase 2 (Row Mappers) ──→ Phase 3 (Service Gaps) ──→ Phase 4 (Tests) ──→ Phase 5 (Contracts)
     1-2 hours              2-3 hours                2-3 hours                4-6 hours           1-2 hours
```

**Total estimated effort**: 10-16 hours

## Risk Assessment

| Phase | Risk | Mitigation |
|-------|------|------------|
| 1. Dead code removal | LOW | Files are unused; `cargo check` validates |
| 2. Row mapper consolidation | MEDIUM | Each migration is mechanical; `cargo check` + existing tests validate |
| 3. Service gaps | MEDIUM | Schema validation is straightforward; feed composition is the most complex piece |
| 4. Tests | LOW | Tests don't affect production code |
| 5. Contract sync | LOW | Documentation-only changes |

## Dependencies

- Phase 2 depends on Phase 1 (clean workspace)
- Phase 3 depends on Phase 2 (row mappers available for feed composition queries)
- Phase 4 depends on Phases 2+3 (tests cover the new code)
- Phase 5 depends on Phase 4 (contract tests must pass with new assertions)

