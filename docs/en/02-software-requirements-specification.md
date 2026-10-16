# DOC-02 — Software requirements specification

| Field | Value |
| --- | --- |
| Status | Approved |
| Version | 0.1.0 |
| Product target | MVP 0.1.0 |
| Last updated | 2026-10-16 |

## 1. Conventions

**MUST**, **MUST NOT**, **SHOULD**, and **MAY** express requirement strength. IDs are permanent.

## 2. Functional requirements

| ID | Requirement | Verification |
| --- | --- | --- |
| RF-001 | The CLI MUST execute `sonvik run <request>` with an optional environment. | E2E |
| RF-002 | It MUST accept GET, POST, PUT, PATCH, DELETE, HEAD, and OPTIONS. | Integration |
| RF-003 | It MUST support URL, headers, and `body.json`. | Integration |
| RF-004 | It MUST reject invalid documents and unknown fields before sending. | Unit/E2E |
| RF-010 | It MUST load variables from an optional YAML environment. | Unit |
| RF-011 | It MUST resolve `{{name}}` placeholders in supported textual values. | Unit |
| RF-012 | Precedence MUST be `--var` > process environment > environment file. | Unit |
| RF-013 | A missing variable or invalid placeholder MUST prevent sending. | Unit/E2E |
| RF-014 | `--var` MUST be documented as unsuitable for secrets. | Review |
| RF-020 | It MUST compare the received status with `expect.status`. | Integration |
| RF-021 | It MUST return `0` when the assertion passes and `1` when it fails. | E2E |
| RF-022 | It MUST return `2` for invalid usage, configuration, or execution. | E2E |
| RF-023 | It MUST visibly distinguish success, assertion failure, and execution error. | E2E |
| RF-030 | It MUST apply a finite timeout with an explicit override. | Integration |
| RF-031 | It MUST NOT retry automatically. | Integration |
| RF-032 | It MUST follow up to 5 redirects and reject HTTPS→HTTP downgrades. | Integration |
| RF-040 | It MUST NOT print headers or bodies by default. | Security |
| RF-041 | It MUST accept secrets through environment variables without persisting them. | Security |
| RF-042 | Errors MUST avoid repeating resolved sensitive values. | Security |

## 3. Non-functional requirements

| ID | Category | Requirement | Verification |
| --- | --- | --- | --- |
| RNF-001 | Portability | Core and CLI MUST compile on macOS, Linux, and Windows. | CI |
| RNF-002 | Reproducibility | Tests MUST run without external network access. | CI |
| RNF-003 | Format | Every persistent document MUST declare `version: 1`. | Unit |
| RNF-004 | Maintainability | Logic MUST be usable without depending on the CLI. | Architecture/test |
| RNF-005 | Security | Secrets MUST NOT appear in logs, errors, or fixtures. | Audit/test |
| RNF-006 | Performance | Excluding remote latency, a run SHOULD add less than 100 ms. | Later benchmark |
| RNF-007 | Compatibility | Breaking schema changes MUST change the version and document migration. | Release gate |
| RNF-008 | Distribution | The published binary MUST NOT require an external runtime. | Smoke test |

## 4. Traceability rules

- Every use case references its `RF-*` requirements.
- Every DOC-04 component states responsibilities without duplicating requirements.
- Every persistent datum appears in DOC-05.
- Every DOC-06 milestone has an observable gate.

## 5. Document history

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | First verifiable specification |
