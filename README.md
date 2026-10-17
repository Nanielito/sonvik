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

## Project documentation

- [Documentation](https://nanielito.github.io/sonvik/)
- Project brief: [English](docs/en/project-brief.md) · [Español](docs/es/project-brief.md)
- ADR-001: [English](docs/adr/ADR-001-mvp-architecture-en.md) · [Español](docs/adr/ADR-001-mvp-architecture.md)

## License

Licensed under either [Apache License 2.0](LICENSE-APACHE) or the [MIT License](LICENSE-MIT), at your option.
