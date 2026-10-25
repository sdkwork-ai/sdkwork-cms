# CMS Contract Skeleton Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a SDKWork-compliant CMS backend contract skeleton with database, API, SDK, and Rust module boundaries ready for implementation by follow-up agents.

**Architecture:** CMS owns content modeling, entries, pages, publishing snapshots, feeds, audit, and outbox contracts. IAM, Drive, Search, Messaging, Comments, and Engagement remain dependency-owned and are consumed through SDK/service contracts rather than copied tables or APIs.

**Tech Stack:** Rust crate skeletons, SQL migration contracts, OpenAPI 3.1.2 contracts, SDKWork SDK family metadata, SDKWork repository/application standards.

---

### Task 0: Architecture And Contract Registries

**Files:**
- Create: `docs/architecture/cms-backend-architecture.md`
- Create: `docs/database/cms-v1-table-registry.json`
- Create: `docs/database/cms-v1-column-registry.json`
- Create: `docs/api/cms-v1-operation-registry.json`
- Create: `docs/security/cms-v1-permission-registry.json`
- Create: `docs/integration/cms-v1-integration-registry.json`

- [x] Document SDKWork CMS backend directory ownership and module boundaries.
- [x] Define machine-readable database table registry for all 22 CMS-owned tables.
- [x] Define machine-readable database column registry for every migration column.
- [x] Define machine-readable API operation registry for all 76 Open/App/Backend operations.
- [x] Define machine-readable permission registry for all 22 backend permissions.
- [x] Define machine-readable event, outbox, and dependency-port integration registry.
- [ ] TODO: Wire registries into official SDKWork validators when available.

### Task 1: Root Workspace Skeleton

**Files:**
- Create: `AGENTS.md`
- Create: `agent.app.config.json`
- Create: `.sdkwork/README.md`
- Create: top-level directory README placeholders

- [x] Create SDKWork application root metadata.
- [x] Document active layout and dependency boundaries.
- [ ] TODO: Run official app manifest validator when available in this repository.

### Task 2: Database Contract

**Files:**
- Create: `docs/database/cms-v1-schema.md`
- Create: `crates/sdkwork-content-cms-repository-sqlx/migrations/0001_cms_v1_foundation.sql`
- Create: repository crate schema constants

- [x] Define the CMS v1 table catalog.
- [x] Create SQL migration draft for all v1 tables, constraints, and indexes.
- [x] Add field-level database contract registry generated from the migration.
- [ ] TODO: Add schema linter integration once SDKWork database validation tooling is wired.

### Task 3: API Contracts

**Files:**
- Create: `apis/open-api/content/cms-open-api.openapi.json`
- Create: `apis/app-api/content/cms-app-api.openapi.json`
- Create: `apis/backend-api/content/cms-backend-api.openapi.json`

- [x] Define open-api delivery operations.
- [x] Define app-api delivery operations.
- [x] Define backend-api management operations.
- [ ] TODO: Materialize route manifests from Rust route crates into authority OpenAPI during implementation.

### Task 4: Rust Backend Module Skeleton

**Files:**
- Create: `crates/sdkwork-content-cms-service/**`
- Create: `crates/sdkwork-content-cms-repository-sqlx/**`
- Create: `crates/sdkwork-routes-cms-open-api/**`
- Create: `crates/sdkwork-routes-cms-app-api/**`
- Create: `crates/sdkwork-routes-cms-backend-api/**`

- [x] Define domain structs, commands, result types, service ports, and service method signatures.
- [x] Define repository implementation placeholders.
- [x] Split service and repository methods by capability module for follow-up implementation agents.
- [x] Define route path constants, route manifests, handlers, mappers, and TODOs.
- [x] Split route handler TODOs by API capability and add DTO request/response skeleton modules.
- [ ] TODO: Implement service use cases with tests in a follow-up task.

### Task 5: SDK Family Skeleton

**Files:**
- Create: `sdks/sdkwork-cms-sdk/**`
- Create: `sdks/sdkwork-cms-app-sdk/**`
- Create: `sdks/sdkwork-cms-backend-sdk/**`

- [x] Define SDK assembly metadata and component specs.
- [x] Mirror authority OpenAPI source locations.
- [ ] TODO: Run canonical `sdkgen --standard-profile sdkwork-v3` after OpenAPI review.

### Task 6: Verification

**Files:**
- Create: `tests/static/cms-contract-boundary.test.mjs`

- [x] Add static checks for required tables, migration columns, OpenAPI paths, route crate names, route manifests, API operation counts, service/repository method boundaries, permissions, integrations, handler modules, DTO modules, and SDK assemblies.
- [x] Run static verification.
- [x] Document residual implementation TODOs.
