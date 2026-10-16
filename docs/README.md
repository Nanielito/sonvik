# Sonvik

**Explore. Execute. Verify.**

Sonvik is an open-source tool for executing and verifying APIs from readable, versionable files. It runs locally, requires no account, and produces reproducible results in both development and CI.

## Why Sonvik

- **Local-first:** your requests and configuration remain in your files.
- **Git-native:** tests are reviewed and versioned alongside the code.
- **Automation-ready:** the same command works locally and in CI.
- **No GUI or cloud dependency:** the MVP starts with a small, portable CLI.

## Project status

Sonvik is building its `0.1.0` MVP: HTTP requests defined in YAML, optional environments, variable substitution, status verification, and stable exit codes.

[Documentación en español](es/README.md) · [Read the English documentation](en/README.md) · [View the project on GitHub](https://github.com/Nanielito/sonvik)

## Documentation

- **Documentation version:** 0.1.0
- **Target product:** MVP 0.1.0
- **Canonical language:** Spanish; synchronized English translation
- **Last synchronized:** 2026-10-16

This directory contains Sonvik's living documentation. The original brief ([ES](es/project-brief.md) · [EN](en/project-brief.md)) preserves the initial exploration; these documents turn that vision into traceable scope, requirements, and decisions.

## Documentation map

| ID | Español | English | Purpose |
| --- | --- | --- | --- |
| BRIEF | [Brief del proyecto](es/project-brief.md) | [Project brief](en/project-brief.md) | Initial context and non-normative proposals |
| DOC-01 | [Visión y alcance](es/01-vision-y-alcance.md) | [Vision and scope](en/01-vision-and-scope.md) | Problem, value proposition, and boundaries |
| DOC-02 | [Especificación de requisitos](es/02-especificacion-de-requisitos.md) | [Software requirements](en/02-software-requirements-specification.md) | Verifiable MVP behavior |
| DOC-03 | [Casos de uso](es/03-casos-de-uso.md) | [Use cases](en/03-use-cases.md) | Interactions and alternative flows |
| DOC-04 | [Arquitectura de software](es/04-arquitectura-de-software.md) | [Software architecture](en/04-software-architecture.md) | Boundaries, flow, deployment, and security |
| DOC-05 | [Diccionario de datos](es/05-diccionario-de-datos.md) | [Data dictionary](en/05-data-dictionary.md) | Minimal canonical model and validation rules |
| DOC-06 | [Roadmap y gobernanza](es/06-roadmap-y-gobernanza.md) | [Roadmap and governance](en/06-roadmap-and-governance.md) | Delivery, quality, risks, and change management |
| ADR-001 | [Arquitectura y alcance del MVP](adr/ADR-001-mvp-architecture.md) | [MVP architecture and scope](adr/ADR-001-mvp-architecture-en.md) | First architecture decision |

## Statuses

`Draft` → `In review` → `Approved` → `Superseded` or `Retired`.

## Rules

1. Spanish is the canonical source; the English version is synchronized in the same change.
2. IDs are not reused; a removed item changes status.
3. A behavior change updates the affected requirements, use cases, and criteria.
4. Significant architecture decisions are recorded in `docs/adr/`.
5. Markdown is the source of truth; other formats are generated artifacts.
