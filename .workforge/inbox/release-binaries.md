# Sonvik release binaries

This requirement completes the binary-distribution portion of the MVP release gate. GitHub Releases in the existing repository are the artifact store; versioning, changelog generation, tags, and crates.io publication are tracked separately and remain out of scope for this item.

## Publish portable binaries in the GitHub Release

Source: product_requirements
Priority: high
Milestone: v0.1.0
Labels: ci, maintenance, docs

Use cargo-dist to build and attach ready-to-run Sonvik binaries to the GitHub Release created for the release-plz tag. Do not create a separate repository or use GitHub Packages.

- Add the minimal cargo-dist configuration and generated release workflow
- Configure cargo-dist to own GitHub Release creation so it does not conflict with release-plz
- Build archives for Linux x86_64, macOS Apple Silicon, and Windows x86_64
- Generate and publish SHA-256 checksums beside every binary archive
- Validate that the tagged Cargo version matches the release version before publishing artifacts
- Keep the release unpublished when any required build or checksum step fails
- Document installation through `cargo install sonvik --locked` and direct GitHub Release downloads
- Close OQ-006 in DOC-06 and record the supported release target matrix in synchronized Spanish and English documentation
- Add the release checks to the MVP traceability and release gate without expanding product behavior
- Verify the release plan locally without creating a tag, GitHub Release, or crates.io version
- Defer Homebrew, Scoop, WinGet, containers, graphical installers, code signing, notarization, and additional architectures until demand justifies them

