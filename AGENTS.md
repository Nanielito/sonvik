# Sonvik Agent Guide

## Project overview

Sonvik is an open-source, local-first, Git-native CLI for executing and verifying APIs from versioned files. The current product is the HTTP verification MVP, not yet an API exploration client.

The approved architecture and scope live in `docs/adr/ADR-001-mvp-architecture.md` and its English counterpart. Treat DOC-01 through DOC-06 as the normative product baseline; the project briefs preserve historical context and are not specifications.

## Current boundaries

The MVP supports one declarative HTTP request with:

- strict YAML `version: 1` request and environment documents;
- GET, POST, PUT, PATCH, DELETE, HEAD, and OPTIONS;
- headers and optional `body.json`;
- `{{name}}` substitution in URL, header values, and JSON string values;
- variable precedence: `--var` > process environment > environment file;
- exact status assertion;
- finite timeout and bounded redirects;
- human output and exit codes `0`, `1`, and `2`.

Flows, captures, body assertions, scripting, importers, structured reports, GUI, MCP, GraphQL-specific behavior, gRPC, WebSocket, retries, and concurrency are out of scope until approved separately.

## Repository map

- `src/lib.rs`: reusable load, validation, resolution, HTTP, and evaluation path.
- `src/main.rs`: CLI parsing, presentation, and process exit codes.
- `tests/cli.rs`: offline CLI end-to-end coverage using a local ephemeral server.
- `examples/`: versionable request and environment examples.
- `docs/es/`: canonical Spanish product documentation.
- `docs/en/`: synchronized English translation.
- `docs/adr/`: architecture decision records in both languages when applicable.
- `.workforge/inbox/`: provider-neutral planning requirements.
- `.github/workflows/`: CI and documentation publishing.
- `release-plz.toml` and `CHANGELOG.md`: release preparation and release history.

Do not commit `target/`, `site/`, `.workforge/.env`, or `.workforge/output/`.

## Working rules

- Read the affected flow and every caller before changing shared behavior.
- Prefer one package, direct code, standard library, and existing dependencies. Add a crate or abstraction only when the current requirement needs it.
- Keep the library independent from terminal presentation and exit codes.
- Reject invalid configuration before sending traffic.
- Never print request headers, bodies, resolved variables, tokens, or secrets by default.
- Treat YAML, arguments, environment values, URLs, redirects, certificates, and responses as untrusted input.
- Preserve unknown user changes and avoid unrelated cleanup.
- Do not implement deferred product scope without an approved requirement and architecture review.

## Development and validation

Rust 1.89 or newer is required. Before handing off a code change, run:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
git diff --check
```

Tests must remain offline. Network behavior is tested with a server bound to `127.0.0.1` on an ephemeral port. Add the smallest executable test that distinguishes new behavior or prevents a regression.

For documentation changes, also run:

```bash
python -m mkdocs build --strict
```

If a command cannot run, report exactly which check was skipped and why.

## Git and pull requests

- Use Conventional Commits: `type(optional-scope): imperative summary`.
- Use `feat/<issue>-<slug>`, `fix/<issue>-<slug>`, `docs/<issue>-<slug>`, or `chore/<issue>-<slug>` from an up-to-date `main`.
- A single coherent vertical may cover multiple tightly coupled issues in one branch and PR; do not stack branches without an independent integration need.
- Do not commit, push, close issues, or open/merge a PR unless the user asks.
- Leave tracked issues in progress while their PR is open. Close them through the merged PR or the repository's project automation.
- Never stage `.workforge/.env` or generated provider output.

## Workforge workflow

Requirements are written in `.workforge/inbox/<requirement>.md`, with one `## Title` section per issue. Preview before external writes:

```bash
uvx workforge preview .workforge/inbox/<requirement>.md --workspace .workforge
```

Create issues only with explicit authorization:

```bash
uvx workforge create .workforge/inbox/<requirement>.md --workspace .workforge --execute --save
```

Before implementation, claim the issue and move it to `doing`. Treat its managed task list as the implementation checklist. Complete each task through Workforge after its relevant check passes. Do not mark remote CI, deployment, or manual verification complete by inference.

Keep `.workforge/workforge.yaml`, `.workforge/.env.example`, `.workforge/README.md`, and inbox requirements versioned. Keep credentials and generated output local.

## Releases

- release-plz owns version changes, `CHANGELOG.md`, `v<semver>` tags, and crates.io publication.
- Only a merged `release-plz-*` pull request may publish; ordinary pushes must remain non-publishing.
- The GitHub Release and downloadable binaries belong to the separate distribution workflow.
- Never merge a release pull request until all required distribution and release checks are available and passing.

## Keep in sync

Every behavioral change must update the smallest applicable set below in the same pull request:

| Change | Keep synchronized |
| --- | --- |
| Product scope or user-visible behavior | `docs/es/01-*`, `docs/en/01-*`, requirements, and use cases |
| Functional or non-functional requirement | Matching IDs in both DOC-02 editions and referenced use cases |
| CLI syntax, exit codes, or examples | `README.md`, `src/main.rs`, `examples/`, tests, and both language docs |
| YAML schema, validation, defaults, or limits | `src/lib.rs`, DOC-02, DOC-05 in both languages, examples, and tests |
| Architecture or dependency boundary | DOC-04 in both languages and a new/superseding ADR when significant |
| Milestone, risk, or open decision | DOC-06 in both languages and the relevant Workforge requirement |
| Planning checklist or implementation scope | `.workforge/inbox/` and the provider checklist through Workforge |
| Documentation navigation | `mkdocs.yml` and the relevant documentation indexes |

Spanish is canonical, but English must be updated in the same change. Preserve existing `RF-*`, `RNF-*`, `CU-*`, `DAT-*`, `RSK-*`, and `OQ-*` identifiers; retire IDs instead of reusing them.

## Definition of done

A change is done when its accepted behavior and relevant errors are implemented, proportional tests pass, secrets remain redacted, bilingual documentation and Workforge tracking are synchronized, generated/local state is excluded, and the handoff includes verification evidence plus a PR-ready title and summary.
