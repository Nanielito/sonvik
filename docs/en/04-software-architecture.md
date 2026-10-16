# DOC-04 — Software architecture

| Field | Value |
| --- | --- |
| Status | Approved |
| Version | 0.1.0 |
| Product target | MVP 0.1.0 |
| Last updated | 2026-10-16 |

## 1. Drivers

- Identical behavior locally and in CI.
- Portable binary with no Sonvik service.
- Readable, versionable files.
- Safe errors before effects whenever possible.
- A future path to more interfaces without designing them today.

## 2. Structural decision

One Cargo package contains a library and a binary:

```text
sonvik/
├── Cargo.toml
├── src/
│   ├── lib.rs       # reusable pipeline
│   └── main.rs      # CLI, presentation, and exit code
├── examples/
├── tests/
└── docs/
```

`main` depends on the library; the library knows nothing about arguments, terminals, or exit codes. There are no traits for services with one implementation and no crate per module.

## 3. Flow

```text
CLI -> load/validate -> variables -> HTTP request -> evaluate -> result
```

| Responsibility | Input | Output |
| --- | --- | --- |
| Load | paths | deserialized documents |
| Validate | documents | valid model or located errors |
| Resolve | model + sources | resolved request or missing variable |
| HTTP | request + timeout | response metadata or transport error |
| Evaluate | actual/expected status | pass/fail result |
| Present | result/error | text and exit code |

These are responsibilities, not mandatory modules. Implementation uses the fewest files that remain readable.

## 4. Trust boundaries

YAML, environment variables, arguments, DNS, certificates, and every response are untrusted. Before sending, Sonvik validates version, fields, method, URL, placeholders, and timeout. Each declarative file is limited to 1 MiB.

## 5. Security

- Environment files are not secret stores by convention and documentation.
- Secrets enter through the process environment.
- `--var` is only for non-sensitive values.
- Normal output omits headers and bodies.
- Errors name variables, not resolved values.
- There are no hooks, scripts, or expression evaluation.
- Redirect and TLS policies are explicit and tested.

## 6. Errors

The library distinguishes at least:

- configuration/validation;
- variable resolution;
- transport/timeout;
- assertion failure.

Only the last represents a technically valid execution with a negative verification. `main` maps categories to exit codes according to ADR-001.

## 7. Portability and deployment

The artifact is one binary per platform. Once code exists, CI will run formatting, linting, tests, and builds on native runners. The MVP does not promise installers; a release may publish binaries and checksums.

## 8. Evolution

- Real second consumer → consider extracting a core crate.
- Real second structured output → minimal reporter interface.
- Real second transport → extract a contract from both implementations.
- Real chaining → design Flow, scopes, and captures in another ADR.

## 9. Related decisions

- [ADR-001](../adr/ADR-001-mvp-architecture.md): MVP scope, single package, CLI, and format.

## 10. Document history

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Initial pragmatic architecture |
