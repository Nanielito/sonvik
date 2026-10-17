# DOC-06 — Roadmap, quality, and governance

| Field | Value |
| --- | --- |
| Status | Approved |
| Version | 0.1.0 |
| Product target | MVP 0.1.0 |
| Last updated | 2026-10-16 |

## 1. Strategy

Deliver executable verticals. The roadmap expresses order and gates, not dates or promises for future versions.

## 2. Milestones

### M0 — Documentation baseline

- Approve or revise DOC-01 through DOC-06 and ADR-001.
- Close pending decisions for the minimal schema.

**Gate:** no open contradictions about scope, CLI, format, or exit codes.

### M1 — First vertical

- Rust package with library + binary.
- Load/validation, variables, HTTP, status assertion, and exit codes.
- E2E test with a local server.

**Gate:** CU-001, CU-002, and CU-004 pass offline.

### M2 — Portability and release 0.1.0

- CI on macOS, Linux, and Windows.
- CU-003/CU-005, usage documentation, and examples.
- Portable archives and SHA-256 checksums for Linux x86_64, macOS Apple Silicon, and Windows x86_64.

**Gate:** every MVP Must has evidence, native release jobs produce the three required archives and checksums, and fixtures/output contain no secrets.

### After 0.1.0

Real usage selects the next problem. Independent candidates include JSON assertions, collections/flows, structured output, import, or desktop. No versions are assigned yet.

## 3. Definition of done

A capability is done when it:

- implements its primary flow and relevant errors;
- has the minimum test proportional to risk;
- does not leak secrets;
- works offline when no external API is required;
- updates affected requirements, cases, data, and ADRs;
- documents visible limits.

## 4. Test strategy

- Unit tests for validation, precedence, and placeholders.
- Integration tests for methods, bodies, status, redirects, and timeout.
- One CLI E2E test against an ephemeral local server.
- Security cases for redaction and no sending after local errors.
- Smoke builds per operating system.
- A distribution plan that validates the release tag, target matrix, archives, and checksums before publication.

No percentage coverage target is set; evidence is required for branches with distinct behavior.

## 5. Risks

| ID | Risk | Impact | Treatment |
| --- | --- | ---: | --- |
| RSK-001 | Rust slows the first vertical | Medium | Review after M1; change only with evidence |
| RSK-002 | YAML is ambiguous or hard to evolve | High | Strict schema, version, and invalid examples |
| RSK-003 | Secret leakage | High | Environment, minimal output, and redaction tests |
| RSK-004 | Scope grows into a Postman clone | High | DOC-01 and gates as filters |
| RSK-005 | HTTP behavior differs across platforms | Medium | Integration tests and native CI |
| RSK-006 | Abstractions for future consumers | Medium | Extract only after a second implementation |

## 6. Resolved decisions

| ID | Decision | Resolution |
| --- | --- | --- |
| OQ-006 | Minimum OS/architecture matrix | Linux x86_64, macOS Apple Silicon, and Windows x86_64 for MVP releases |

## 7. Versioning

- Product and documentation use independent SemVer.
- `0.x` permits documented breaking changes.
- Every canonical file declares a schema version.
- `CHANGELOG.md` is created with the first product change, not before.

## 8. ADRs

Path: `docs/adr/ADR-NNN-title.md`. States: Proposed, Accepted, Rejected, Superseded, Retired.

An ADR includes context, options, decision, consequences, validation, and review conditions. An accepted decision is not substantially rewritten; a new record supersedes it.

## 9. Documentation governance

1. Spanish is canonical; English is synchronized in the same change.
2. Every modification updates the affected document's date, version, and history.
3. Retired IDs are never reused.
4. A normative change reviews traceability and impact.
5. An approved baseline is tagged with its corresponding product.
6. IDs and normative meaning must match between languages.

## 10. Document history

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Initial roadmap and governance |
