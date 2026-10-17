# Sonvik release versioning

This requirement completes the versioning and registry-publication portion of the MVP release gate. Release automation must preserve a human approval point and publish the Rust package through the ecosystem's standard registry. Binary distribution is tracked separately and remains out of scope for this item.

## Automate versioning, changelog, and crates.io publication

Source: product_requirements
Priority: high
Milestone: v0.1.0
Labels: ci, maintenance, docs

Use Conventional Commits as the source for a release pull request that updates the package version and changelog. Merging that reviewed pull request may create the version tag and publish Sonvik to crates.io; ordinary pushes must never publish a release.

- Add the minimal release-plz configuration and GitHub Actions workflow for release pull requests
- Generate and version `CHANGELOG.md` from Conventional Commits using release-plz's git-cliff integration
- Keep version changes in `Cargo.toml` and `Cargo.lock` reviewable in the release pull request
- Configure release-plz to own versioning, the `v<semver>` tag, and crates.io publication, but not the GitHub Release
- Grant GitHub Actions only the permissions required by the release workflow and keep registry credentials out of the repository
- Complete the Cargo package metadata required for crates.io, including repository, homepage, README, license, keywords, and categories
- Restrict publication to crates.io and verify the exact package contents with `cargo package --list`
- Pass `cargo publish --dry-run` before enabling publication
- Document the one-time crates.io owner or trusted-publisher setup that Daniel Ramirez must perform
- Verify that non-release pushes and unmerged release pull requests cannot publish a crate or create a version tag

