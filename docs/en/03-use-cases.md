# DOC-03 — Use cases

| Field | Value |
| --- | --- |
| Status | Approved |
| Version | 0.1.0 |
| Product target | MVP 0.1.0 |
| Last updated | 2026-10-16 |

## 1. Actors

- **User:** creates files and invokes the CLI.
- **CI:** invokes the CLI and consumes exit codes.
- **Target API:** receives requests and returns responses; it is untrusted.
- **Operating system:** provides files, environment variables, network access, and time.

## 2. Catalog

| ID | Use case | Requirements |
| --- | --- | --- |
| CU-001 | Run a request | RF-001–004, RF-020–023, RF-030–032 |
| CU-002 | Run with an environment | RF-010–014 |
| CU-003 | Verify an API in CI | RF-021–023, RNF-001–002 |
| CU-004 | Diagnose invalid configuration | RF-004, RF-013, RF-023 |
| CU-005 | Use a secret without persisting it | RF-040–042, RNF-005 |

## 3. CU-001 — Run a request

**Precondition:** a valid `version: 1` request file exists.  
**Trigger:** `sonvik run request.yaml`.

1. Sonvik loads and validates the document.
2. It resolves available variables.
3. It builds the request and applies a timeout.
4. It sends exactly once.
5. It compares the status.
6. It displays a summary and exits with `0` or `1`.

**Alternatives:** invalid file or method → do not send and return `2`; timeout, DNS, TLS, or connection failure → return `2`; unexpected status → return `1`.

## 4. CU-002 — Run with an environment

1. The user passes `--env environment.yaml` and optionally `--var key=value`.
2. Sonvik validates documents and arguments.
3. It combines sources using defined precedence.
4. It replaces every placeholder or fails before sending.
5. It continues with CU-001.

**Alternatives:** key in multiple sources → highest precedence wins; missing variable → error `2` without traffic.

## 5. CU-003 — Verify in CI

1. The pipeline checks out the repository and injects secrets through its environment.
2. It runs the same command used locally.
3. It uses the exit code to pass or fail the job.

**Postcondition:** Sonvik creates no account, remote state, or implicit artifact.

## 6. CU-004 — Diagnose invalid configuration

1. Sonvik reports the file, field, and cause when available.
2. It omits resolved sensitive values.
3. It sends no partial traffic.

## 7. CU-005 — Use a secret without persisting it

1. The user or CI defines an environment variable.
2. The file references its name through a placeholder.
3. Sonvik uses the value only during execution.
4. Normal output reports the result without headers or bodies.

## 8. Document history

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Use cases for the first vertical |
