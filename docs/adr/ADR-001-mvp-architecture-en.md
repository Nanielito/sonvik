# ADR-001: MVP architecture and scope

- Status: Accepted
- Date: 2026-10-16

## Context

Sonvik aims to execute and verify HTTP requests defined in local files, both for people and CI. It may later add a desktop application and an MCP interface, but today there is neither a second engine consumer nor evidence that justifies designing those interfaces.

The main risk is not that the MVP has too few layers, but that it prematurely fixes a domain model, format, or repository structure that must then be maintained. The first increment must validate one promise: a versionable request produces the same result locally and in CI.

## Decision

### Functional scope

The MVP will have one vertical path:

```text
request file + optional environment
  -> parse and validate
  -> resolve variables
  -> execute HTTP
  -> compare exact expected status
  -> human output + exit code
```

It will support:

- common HTTP methods;
- URL, headers, and JSON body;
- environments as flat string maps;
- `{{name}}` placeholders without an expression language;
- one assertion: exact HTTP status;
- a finite, configurable timeout;
- concise human output and stable exit codes.

Variable precedence will be `--var` > process environment > environment file. A missing variable or malformed placeholder is an error before sending.

Environment files are configuration, not secret stores. Secrets enter through environment variables; `--var` is reserved for non-sensitive values because arguments may remain in shell history or be visible to other processes. Sonvik will not print headers or bodies by default. The MVP will not build keychain integration or its own secret system.

### Program shape

Sonvik will use Rust, subject to a short review after completing the vertical slice. The repository will begin as **one Cargo package** with:

- `src/lib.rs`: load, validation, resolution, HTTP, and evaluation;
- `src/main.rs`: arguments, result presentation, and exit code.

The library will not depend on the CLI. It will be split into crates only when a real second consumer or build-time/ownership pressure justifies it. Internal modules may follow flow responsibilities without turning every concept into a public entity.

No formal hexagonal architecture or traits for single implementations will be introduced. The useful boundary is simple: logic returns data and errors; `main` decides how to display them and terminate.

### CLI

There will be one execution command:

```bash
sonvik run request.yaml [--env environment.yaml] [--var key=value]
```

`send`, `run`, and `test` will not be separate yet: for one request they represent the same work. Initial exit codes are:

- `0`: request executed and assertion satisfied;
- `1`: request executed but assertion failed;
- `2`: invalid usage, invalid file/configuration, or execution failure.

Structured output such as JSON or JUnit will be decided when the first concrete consumer exists; the MVP will not anticipate a reporter system.

### File format

The MVP will use YAML because people write these files and the current examples already use it. Every document will contain `version: 1`; unknown fields will be rejected and arbitrary extensions will not be accepted. Files are limited to 1 MiB.

Minimal conceptual schema:

```yaml
version: 1
name: Health check
request:
  method: GET
  url: "{{baseUrl}}/health"
  headers:
    Accept: application/json
expect:
  status: 200
```

```yaml
version: 1
variables:
  baseUrl: http://localhost:8080
```

The canonical format is the validated model, not parser implementation details. Any incompatible evolution requires another version and an explicit migration; drafts without `version` receive no compatibility promise.

### Execution and tests

Execution will be sequential with no automatic retries. Retries may repeat non-idempotent operations and hide failures; they will be added only with explicit semantics.

The default timeout is 30 seconds. Sonvik follows up to five redirects and rejects HTTPS-to-HTTP downgrades. Variable names use `[A-Za-z_][A-Za-z0-9_]*`; substitution applies to URL, header values, and strings inside `body.json`, never to keys.

An integration test will start an ephemeral local HTTP server and exercise the complete path. Tests will not depend on an external network, accounts, or cloud services.

## Challenged proposals

| Brief proposal | MVP decision | Reason |
| --- | --- | --- |
| Workspace with `sonvik-core` and `sonvik-cli` | One package with library + binary | Preserves the boundary without workspace or cross-crate API overhead. |
| `Project` as root | Does not exist yet | Running one file needs neither a manifest nor an aggregate model. |
| Requests, collections, and flows | One request only | A collection can later be a list of paths; flows require their own semantics. |
| Assertions, captures, and expressions | Exact status only | JSONPath, typing, and scopes multiply decisions before validating the runner. |
| cURL/OpenAPI import in v0.1 | Deferred | Import requires a stable target format first. |
| Extensible transport layer | Direct HTTP | GraphQL uses HTTP; gRPC/WebSocket will justify abstraction when implemented. |
| Desktop and MCP shape the core | Library/binary boundary only | Preserves reuse without designing APIs for hypothetical consumers. |
| Built-in secure secret management | External injection and minimal logs | Avoids storing secrets and delegates custody to the existing environment. |
| Multiple reporters | Text and exit codes | Minimum needed for local and CI use. |

## Alternatives considered

### Workspace from the start

It makes the core/CLI separation visible but adds manifests, versioning, and a public boundary that provides no real isolation yet. Rejected for now.

### TypeScript/Node as the only implementation

It reduces distance to a future UI but introduces a runtime into CLI distribution. It remains valid if the vertical slice shows that Rust development cost or its HTTP/YAML ecosystem slows the project. Rust is a reversible choice, not a product principle.

### `.http` files

They are excellent for manual requests but lack a standard representation for environments and assertions. Deferred as a possible import format, not the canonical format.

### JSON or TOML

JSON is unambiguous but less convenient for manual editing. TOML works well for configuration but represents bodies and nested HTTP structures less naturally. Both may be reconsidered before declaring `version: 1` stable; the MVP will not support multiple formats.

## Consequences

Positive:

- the first useful result requires few pieces and one architectural boundary;
- core logic and CLI remain decoupled without speculative structure;
- behavior can be verified fully offline;
- versioning the first file prevents accidental compatibility.

Accepted costs and limits:

- there are no flows, captures, body assertions, importers, or GUI yet;
- the CLI does not execute collections;
- Rust may be reassessed after the vertical slice;
- YAML becomes a commitment only when the `version: 1` schema is implemented and documented.

## Review criteria

Review this ADR when any of these events occurs:

- a second engine consumer appears: extracting `sonvik-core` may reduce coupling;
- chained requests are needed: define failure semantics, scope, and captures first;
- a CI consumer requires structured output: add the concrete format it needs;
- another transport is implemented: extract an abstraction from two real implementations;
- the Rust vertical slice reveals distribution or maintenance friction greater than its benefit.
