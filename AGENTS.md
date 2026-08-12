# SDKWork CMS Agent Guide

This repository is the SDKWork CMS application root.

## Canonical Standards

- Workspace rules: `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- Agent execution: `../sdkwork-specs/SOUL.md`
- Standards entrypoint: `../sdkwork-specs/README.md`

## Required Reading By Task

- API work: `../sdkwork-specs/API_SPEC.md`
- SDK work: `../sdkwork-specs/SDK_SPEC.md` and `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`
- Database work: `../sdkwork-specs/DATABASE_SPEC.md`
- Rust backend work: `../sdkwork-specs/RUST_CODE_SPEC.md` and `../sdkwork-specs/WEB_BACKEND_SPEC.md`
- Drive/media integration: `../sdkwork-specs/DRIVE_SPEC.md` and `../sdkwork-specs/MEDIA_RESOURCE_SPEC.md`
- IAM/security integration: `../sdkwork-specs/IAM_SPEC.md` and `../sdkwork-specs/SECURITY_SPEC.md`

## Local Boundaries

- CMS-owned database tables use the `cms_` prefix.
- CMS application-owned APIs live under `apis/`.
- SDK families live under `sdks/` and must remain owner-only.
- Rust service, repository, and route crates live under `crates/`.
- Frontend application work is intentionally out of scope for this repository pass.

## Cross-Domain Dependencies

CMS consumes IAM, Drive, Search, Messaging, Comments, and Engagement through their owning SDKs or service contracts. Do not copy dependency-owned tables, OpenAPI operations, generated SDK code, or route handlers into this application authority.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Int64 Wire Contract (API_SPEC §13.6)

- OpenAPI `int64` fields and parameters `MUST` be `type: string`, `format: int64`,
  a decimal `pattern` such as `^-?[0-9]+$`, and `x-sdkwork-int64-string: true`.
  `type: integer, format: int64` is a contract violation: generated TypeScript
  SDKs then emit `number`, and browsers silently round ids past
  `Number.MAX_SAFE_INTEGER` (2^53), replaying wrong ids into lookups.
- Rust response DTOs `MUST` serialize `i64` wire fields with
  `#[serde(with = "sdkwork_utils_rust::serde_int64")]` (or `::option`); request
  boundaries parse inbound strings with the same helper.
- Generated TypeScript SDKs keep `int64` as `string`; frontend code `MUST NOT`
  convert ids/snowflake ids/sequence ids to `number` for storage, comparison,
  or submission.
- Verification: `node <sdkwork-specs>/tools/check-api-operation-patterns.mjs --workspace .`

