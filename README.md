# Sonvik

A local-first, Git-native tool for executing and verifying APIs from versionable files.

## MVP

```bash
cargo run -- run examples/request.yaml --env examples/environment.yaml
```

```yaml
version: 1
request:
  method: GET
  url: "{{baseUrl}}/health"
expect:
  status: 200
```

Sonvik exits with `0` when the assertion passes, `1` when it fails, and `2` for invalid configuration or execution errors. Secrets belong in process environment variables, never `--var` or versioned files.

Requires Rust 1.89 or newer.

## Installation

Install from crates.io when Rust is available:

```bash
cargo install sonvik --locked
```

Ready-to-run archives and their SHA-256 checksums are also available on the
[latest GitHub Release](https://github.com/Nanielito/sonvik/releases/latest) for
Linux x86_64, macOS Apple Silicon, and Windows x86_64.

## Project documentation

- [Documentation](https://nanielito.github.io/sonvik/)
- Project brief: [English](docs/en/project-brief.md) · [Español](docs/es/project-brief.md)
- ADR-001: [English](docs/adr/ADR-001-mvp-architecture-en.md) · [Español](docs/adr/ADR-001-mvp-architecture.md)

## License

Copyright 2026 Daniel Ramirez.

Licensed under either [Apache License 2.0](LICENSE-APACHE) or the [MIT License](LICENSE-MIT), at your option.
