# DOC-05 — Data dictionary

| Field | Value |
| --- | --- |
| Status | Approved |
| Version | 0.1.0 |
| Product target | MVP 0.1.0 |
| Last updated | 2026-10-16 |

## 1. Conventions

- Unknown fields are rejected.
- Documents declare `version: 1`.
- Variable maps contain strings.
- An omitted field uses a documented default; `null` is not automatically equivalent to omission.
- The internal model may change without migration while preserving public format and behavior.

## 2. DAT-001 — RequestDocument

| Field | Type | Required | Rule |
| --- | --- | ---: | --- |
| `version` | integer | Yes | Exactly `1` |
| `name` | string | No | Non-empty when present |
| `request` | Request | Yes | Valid Request |
| `expect` | Expectation | Yes | MVP requires status |

## 3. DAT-002 — Request

| Field | Type | Required | Rule |
| --- | --- | ---: | --- |
| `method` | enum/string | Yes | GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS |
| `url` | string | Yes | HTTP(S) URL after resolution |
| `headers` | map<string,string> | No | No invalid names |
| `body.json` | YAML/JSON value | No | Serialized as JSON |
| `timeout_ms` | positive integer | No | Default 30000; range 1–300000 |

`body.json` accepts JSON-compatible values only. Sonvik applies `Content-Type: application/json` unless the user supplied it. Authentication is not inferred.

## 4. DAT-003 — Expectation

| Field | Type | Required | Rule |
| --- | --- | ---: | --- |
| `status` | integer | Yes | 100–599 |

## 5. DAT-004 — EnvironmentDocument

| Field | Type | Required | Rule |
| --- | --- | ---: | --- |
| `version` | integer | Yes | Exactly `1` |
| `variables` | map<string,string> | Yes | Valid, unique names |

An environment is not a secret store. The repository SHOULD ignore any local files users choose to create, but the MVP format will not mark individual values as secret.

## 6. DAT-005 — RunResult

Runtime data, not persisted automatically:

| Field | Type | Description |
| --- | --- | --- |
| `outcome` | enum | passed, assertion_failed, error |
| `status` | integer? | Status when a response exists |
| `duration` | duration? | Measured transport time |
| `error_kind` | enum? | Config, variables, transport, or timeout |
| `message` | string | Redacted summary |

## 7. Valid example

```yaml
version: 1
name: Health check
request:
  method: GET
  url: "{{baseUrl}}/health"
  headers:
    Accept: application/json
  timeout_ms: 5000
expect:
  status: 200
```

## 8. Invalid example

```yaml
# Missing version
request: { method: GET, url: https://example.test }
expect: { status: 200 }
```

## 9. Decisions pending before freezing v1

- Multiple values for the same header.

## 10. Document history

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Initial minimal model |
