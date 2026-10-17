# Sonvik v0.1.0 artifact recovery

The `v0.1.0` crate and tag were published successfully, but the GitHub Release contains no portable archives or checksums because the tag created by release-plz did not trigger the cargo-dist workflow. This requirement completes the existing release without republishing the crate or replacing the tag.

## Restore GitHub Release artifacts and downstream release automation

Source: product_requirements
Priority: high
Milestone: v0.1.0
Labels: ci, maintenance, bug

Make release-plz-created tags trigger binary distribution, provide a controlled recovery path for the existing `v0.1.0` tag, and attach the originally promised artifacts to its existing GitHub Release.

- Update release authentication so tags created by release-plz can trigger the cargo-dist release workflow without broadening repository permissions unnecessarily
- Add the smallest controlled way to run binary distribution for an existing version tag and validate that the requested tag exists and matches the Cargo package version
- Reuse the existing `v0.1.0` tag and GitHub Release; do not republish or yank the crates.io package and do not move or replace the tag
- Build and attach the Linux x86_64, macOS Apple Silicon, and Windows x86_64 archives required by `dist-workspace.toml`
- Publish SHA-256 checksums beside every archive and verify each checksum against the downloaded asset
- Install and execute the macOS Apple Silicon archive directly, confirming the MVP request path succeeds against a local HTTP server
- Confirm the recovered release remains `v0.1.0`, targets commit `815e0f8b07200f0f8e741c8c67975febb5784a55`, and is neither draft nor prerelease
- Add an executable or workflow-level regression check showing that future release-plz tags reach binary distribution
- Update synchronized Spanish and English release documentation if the recovery trigger or operator procedure changes
- Record remote workflow execution and artifact verification as manual checks; do not mark them complete from local validation alone

