# DOC-01 — Vision and scope

| Field | Value |
| --- | --- |
| Status | Approved |
| Version | 0.1.0 |
| Product target | MVP 0.1.0 |
| Last updated | 2026-10-16 |

## 1. Purpose

Define the problem Sonvik solves, its intended users, and the boundaries of the first verifiable product. Testable commitments live in [DOC-02](02-software-requirements-specification.md).

## 2. Problem

API testing often depends on a GUI, a cloud account, or a format that is difficult to review. This reduces reproducibility between development and CI and can lock tests into a vendor.

## 3. Vision

Sonvik is an open-source, local-first, Git-native runner for exploring and verifying APIs through readable files. The same engine may serve other interfaces once real needs emerge.

**MVP promise:** a versioned request produces a deterministic, automatable result both locally and in CI.

## 4. Users

| Actor | Need |
| --- | --- |
| Developer | Run and debug a request without an account or GUI |
| API team | Review tests alongside source code |
| CI pipeline | Obtain reproducible results and exit codes |
| Maintainer | Evolve the format and engine without accidental complexity |

## 5. MVP 0.1.0 scope

- Cross-platform CLI.
- HTTP requests from YAML: method, URL, headers, and JSON body.
- Optional environment and non-sensitive overrides.
- Simple placeholder substitution.
- Exact status assertion.
- Finite timeout.
- Human-readable output and stable exit codes.
- Offline tests against a local server.

## 6. Out of scope

- GUI, MCP, collaboration, and cloud synchronization.
- Projects, collections, flows, captures, and scripting.
- Body assertions, JSONPath, and expression languages.
- cURL, OpenAPI, Postman, or Bruno importers.
- JSON/JUnit reports.
- GraphQL-specific behavior, gRPC, and WebSocket.
- Automatic retries and concurrency.
- A Sonvik secret store.

## 7. Principles

1. User-owned files before vendor accounts.
2. One complete vertical before many partial features.
3. Presentation-independent library without speculative layers.
4. Explicit behavior before magic or ambiguous defaults.
5. Secrets do not live in versioned files or normal output.
6. Every persistent format declares a version.
7. An abstraction follows two real implementations, not a possibility.

## 8. Success indicators

- A new user runs the local example by following the README.
- The same file passes or fails identically locally and in CI.
- Invalid configuration fails before sending traffic.
- No test requires Internet access.
- Default output reveals neither headers nor bodies.

## 9. Uncommitted future

Desktop, MCP, flows, importers, and additional protocols remain part of the vision. Each requires usage evidence and an ADR or scope change before adoption.

## 10. Document history

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Initial MVP baseline |
