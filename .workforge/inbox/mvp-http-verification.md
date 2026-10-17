# Sonvik MVP HTTP verification

This requirement turns the approved MVP baseline (DOC-01 through DOC-06 and ADR-001) into independently trackable delivery items. The MVP verifies one declarative HTTP request; API exploration, flows, captures, body assertions, importers, GUI, and MCP remain out of scope.

## Implement the versioned HTTP request engine

Source: product_requirements
Priority: high
Milestone: v0.1.0
Labels: feature

Implement the reusable library path that loads a strict versioned request, resolves configuration, sends one HTTP request, and evaluates its expected status without cloud dependencies.

- Create one Cargo package with a reusable library independent from CLI presentation
- Load request and environment YAML documents with `version: 1`, reject unknown fields, and enforce the 1 MiB file limit
- Support GET, POST, PUT, PATCH, DELETE, HEAD, and OPTIONS with headers and optional `body.json`
- Resolve placeholders in URL, header values, and JSON string values using `--var` overrides, process environment, then environment file precedence
- Reject invalid methods, URLs, statuses, timeouts, variable names, and unresolved placeholders before sending traffic
- Apply a 30 second default timeout with a 5 minute maximum
- Follow at most five redirects and reject HTTPS-to-HTTP downgrade redirects
- Return typed passed, assertion-failed, configuration, variable, and transport outcomes without exposing resolved sensitive values

## Expose the MVP through the Sonvik CLI

Source: product_requirements
Priority: high
Milestone: v0.1.0
Labels: feature, docs

Provide the minimal human and CI interface for executing one request with deterministic output and exit codes.

- Implement `sonvik run <request.yaml> [--env <environment.yaml>] [--var <name=value>]...`
- Return exit code 0 when the status assertion passes, 1 when it fails, and 2 for usage, configuration, or execution errors
- Print concise PASS or FAIL output without response headers, bodies, or resolved variable values
- Add versionable request and environment examples using `body.json`
- Document usage, exit codes, secret handling, Rust version, and the dual MIT or Apache-2.0 license in the root README

## Verify the MVP locally and across supported platforms

Source: product_requirements
Priority: high
Milestone: v0.1.0
Labels: ci, feature

Leave executable evidence that the request path and CLI contract work offline on every supported operating system.

- Add unit tests for successful interpolation, missing variables, and invalid variable names
- Add an offline end-to-end test with an ephemeral local HTTP server and a JSON request body
- Verify CLI exit codes 0, 1, and 2 through executable tests
- Pass `cargo fmt --check`, Clippy with warnings denied, and all local targets
- Add GitHub Actions jobs for formatting, Clippy, and tests on Linux, macOS, and Windows
- Verify the GitHub Actions matrix succeeds on Linux, macOS, and Windows

