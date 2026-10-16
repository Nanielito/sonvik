# Sonvik — Project brief / Codex handoff

> Status: concept and initial architecture. This document summarizes the decisions and proposals discussed before implementation; it does not imply that code already exists.

## 1. Vision

**Sonvik** is an open-source tool for exploring, executing, and testing APIs. It is inspired by clients such as Postman and Bruno but designed from the start as a **testing engine independent of its interface**.

**Provisional tagline:** *Explore. Execute. Verify.*

**Goal:** provide a local-first, Git-native, automation-oriented alternative with a CLI as its primary interface, a future desktop application, and possible MCP integration for agents.

**Motivation:** avoid mandatory accounts, cloud services, commercial limits, and proprietary formats. The differentiator is not only executing requests, but enabling **reproducible tests and declarative flows** that people, CI/CD systems, and agents can use.

## 2. Product principles

1. **Your APIs, your files:** projects and collections live locally as readable, Git-versionable text files; no mandatory login.
2. **One engine, multiple interfaces:** CLI, desktop, and MCP reuse the same core.
3. **Testing as code:** requests, assertions, variables, captures, and flows are reproducible in automation.
4. **Open, portable format:** avoid lock-in; favor YAML, HTTP files, or another readable format, with importers for existing formats.
5. **Focused MVP:** do not begin with cloud collaboration, synchronization, a marketplace, dashboards, or multiple protocols.
6. **Local security:** secrets must not be versioned or printed accidentally; consider environment variables and native secure stores.

## 3. Users and use cases

- Developers inspecting endpoints manually.
- Teams versioning tests alongside an API's code.
- CI/CD pipelines executing collections or flows without a GUI.
- In the future, AI agents executing tests through an MCP server.

## 4. Proposed architecture

```text
                    +----------------+
                    |  Desktop UI    |  (future)
                    +--------+-------+
                             |
+-------------+              |
| Sonvik CLI  +--------------+
+-------------+              |
                             v
                    +----------------+
                    |  Sonvik Core   |
                    |----------------|
                    | requests       |
                    | environments   |
                    | variables      |
                    | assertions     |
                    | captures       |
                    | flows          |
                    | runner         |
                    | secrets        |
                    +--------+-------+
                             |
                    +--------+---------+
                    |                  |
                    v                  v
                   HTTP          other protocols
                                (GraphQL/gRPC/etc.)

+----------------+       +----------------+
| MCP interface  +------>|  Sonvik Core   |  (future)
+----------------+       +----------------+
```

**Preferred technology to explore:** Rust for the core and CLI; Tauri + TypeScript for desktop. Rust was also proposed as a learning opportunity. **This is not an irrevocable technical decision**: validate ergonomics, distribution, and maintenance cost.

**Separation of responsibilities:** the core must not depend on the UI; interfaces are engine adapters. Consider modular architecture without overdesigning the MVP.

### Suggested initial organization

```text
sonvik/
├── Cargo.toml                 # proposed Rust workspace
├── crates/
│   ├── sonvik-core/
│   └── sonvik-cli/
├── apps/
│   └── desktop/              # future
├── examples/
└── docs/
```

The names `sonvik-core`, `sonvik-cli`, `sonvik-desktop`, and `sonvik-mcp` are **conceptual names**; the final repository structure remains open.

## 5. Domain model to define

Proposed main entities:

| Entity | Responsibility |
| --- | --- |
| `Project` | Root, configuration, references to requests and environments |
| `Request` | Method, URL, headers, query, body, authentication |
| `Environment` | Local/staging/production-specific values |
| `Variable` | Placeholder resolution and scope |
| `Response` | Status, headers, body, duration, and metadata |
| `Assertion` | Declarative response validation |
| `Capture` | Value extraction for later steps |
| `Flow` | Declarative request sequence and dependencies |
| `Runner` | Orchestration, execution, and reporting |

Open questions: variable precedence and scope; expression typing; error management; timeouts; retries; authentication; file schema and versioning; secret security; data flow between steps; concurrency; output; and exit codes.

## 6. Project format: exploratory examples

**These YAML documents are sketches, NOT an approved specification.** A consistent, versioned schema must be designed before implementation.

```text
api-tests/
├── project.yaml
├── environments/
│   ├── local.yaml
│   ├── staging.yaml
│   └── production.yaml
├── auth/
│   └── login.yaml
├── users/
│   ├── create-user.yaml
│   ├── get-user.yaml
│   └── delete-user.yaml
└── flows/
    └── user-lifecycle.yaml
```

Request example:

```yaml
name: Create user
request:
  method: POST
  url: "{{baseUrl}}/users"
  headers:
    Content-Type: application/json
  body:
    name: Daniel
    email: dan@example.com
expect:
  status: 201
  body:
    id: exists
    name: Daniel
capture:
  userId: $.id
```

Flow example:

```yaml
name: user-lifecycle
steps:
  - run: auth/login
    capture:
      token: $.access_token
  - run: users/create
    capture:
      userId: $.id
  - run: users/get
    with:
      id: "{{userId}}"
  - run: users/delete
    with:
      id: "{{userId}}"
```

**Design note:** explicitly define how captures work, whether they belong to each request or to the flow, and what happens when a step fails. The `exists` and `$.id` syntax also requires specification.

## 7. Desired CLI experience

Illustrative commands; names and arguments remain subject to design:

```bash
sonvik init
sonvik send users/create
sonvik run user-lifecycle
sonvik test --env staging
sonvik import openapi.yaml
sonvik run --env staging --report junit
```

Expected properties:

- Same behavior locally and in CI.
- Human-readable output and machine-processable report formats.
- Appropriate exit codes for automation.
- Ability to execute one request, a collection, or a flow.
- Never expose tokens or secrets in logs or reports by default.

## 8. Tentative roadmap

| Phase | Scope |
| --- | --- |
| **v0.1** | HTTP (GET/POST/PUT/PATCH/DELETE), headers, query, JSON/text body, environments, variables, status/JSON assertions, captures, CLI; cURL/OpenAPI import as an initial goal to validate |
| **v0.2** | Declarative flows |
| **v0.3** | Postman collection import |
| **v0.4** | Desktop UI |
| **v0.5** | GraphQL |
| **v0.6** | gRPC and WebSocket |
| **v0.7** | MCP server for agents |

Versions are **guidance, not commitments**. Reduce the first milestone further if needed—for example, begin with HTTP + CLI + status assertions before importing OpenAPI.

## 9. Interoperability and formats

Proposed import sources:

- cURL
- OpenAPI
- Postman Collections
- Bruno Collections

Sonvik's canonical format must be Git-friendly, readable, and independent from proprietary services. Evaluate YAML against `.http` files and other formats before closing the decision.

## 10. MCP and agents: future vision

Sonvik could not only **consume** MCP services; it could also **expose its own engine** as tools for agents.

Conceptual examples:

```text
run_flow("user-lifecycle", env="staging")
run_request("users/create", variables={...})
```

Permissions, secrets, protection against destructive requests, and isolation must be designed before enabling agent execution.

## 11. Name and identity

**Chosen name:** Sonvik.

**Conceptual origin:** the probe/sonar metaphor—send a signal, observe the response, and understand a system's behavior. The name evolved after researching and rejecting Sonda, Kova, Runa, Vexa, Fluxa, Sondr, Pondr, Ekon, and others because of conflicts or proximity to existing products.

**Provisional tagline:** *Explore. Execute. Verify.*

**Important:** Sonvik was selected as a working name, but availability has **NOT been certified** for domains, GitHub organizations, crates.io, npm, PyPI, or trademarks. Verify before publication or definitive branding, including relevant trademark searches such as INPI Argentina and USPTO classes 9 and 42.

## 12. Suggested first Codex tasks

1. Propose **ADR-001** defining MVP scope, decisions, and alternatives.
2. Design the **domain model** and boundaries among core, CLI, HTTP transport, and persistence.
3. Propose the **minimal versioned format** for requests, environments, and assertions, with valid and invalid examples.
4. Create a Rust workspace with `sonvik-core` and `sonvik-cli`, tests, and basic CI.
5. Implement the first vertical path: load a request file → resolve variables → send HTTP → evaluate status → emit result and exit code.
6. Add unit and integration tests with a local/mock HTTP server and no external service dependency.
7. Document pending decisions before implementing flows, importers, or UI.

### Acceptance criteria for the first vertical slice

- A user can define and execute an HTTP request from a versionable file.
- The user can select an environment and resolve `{{baseUrl}}`.
- The user can verify an expected HTTP status.
- The CLI indicates success or failure with a consistent exit code.
- Tests run offline against a local test server.
- No Sonvik account or cloud service is required.

## 13. Suggested prompt for starting with Codex

> We are building **Sonvik**, an open-source, local-first, Git-native API testing tool, with a Rust core/CLI and a future Tauri UI and MCP server. Read `docs/en/project-brief.md` in full. First analyze the open decisions and propose ADR-001, a pragmatic modular architecture, and a minimal request/environment/assertion schema. Do not implement flows, GUI, or MCP yet. After agreeing on the design, create a Rust workspace with `sonvik-core` and `sonvik-cli` and the first vertical slice: execute a declarative HTTP request, resolve variables, verify status, and return correct exit codes. Include local automated tests, documentation, and basic CI. Prioritize simplicity, portability, and zero cloud dependency.

---

