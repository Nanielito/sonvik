# Releasing Sonvik

`release-plz` prepares versions from Conventional Commits. It owns version changes, `CHANGELOG.md`, the `v<semver>` tag, and crates.io publication. `cargo-dist` uses that tag to create the GitHub Release and attach portable archives and SHA-256 checksums.

## One-time setup

Daniel Ramirez must complete these steps in GitHub and crates.io before merging the first release pull request:

1. Sign in to crates.io with GitHub and verify the email address.
2. Create a short-lived crates.io token with `publish-new` and `publish-update` permissions, and save it as the `CARGO_REGISTRY_TOKEN` Actions secret.
3. Create a fine-grained personal access token limited to the `Nanielito/sonvik` repository, with read and write access to Contents and Pull requests, and save it as `RELEASE_PLZ_TOKEN`. Pull requests created with this token trigger the repository CI.
4. In **Settings → Actions → General**, allow GitHub Actions to create pull requests.

The crates.io token is required for the first publication because new crates cannot use Trusted Publishing. After publishing `v0.1.0`, revoke the token, migrate to Trusted Publishing, and remove `CARGO_REGISTRY_TOKEN` in a separate change.

## Release flow

1. A push to `main` runs release-plz, but ordinary commits are not published.
2. release-plz opens or updates a `release-plz-*` pull request with the proposed version, `Cargo.lock`, and changelog.
3. Review the diff and wait for CI to pass.
4. Merging that pull request makes release-plz publish the crate and create the corresponding tag.
5. The tag starts the distribution workflow. The GitHub Release is published only after every required native build and checksum job succeeds.

`release_always = false` limits publication and tag creation to commits from the release pull request. An open pull request or an ordinary push does not publish.

Release archives are built for Linux x86_64, macOS Apple Silicon, and Windows x86_64. Installers, package-manager formulas, signing, and additional architectures are not part of the MVP.

## Local validation

Before enabling publication:

```bash
cargo package --list
cargo publish --dry-run
dist plan --tag v0.1.0
```

Publication is permanent: an existing version cannot be overwritten or deleted from crates.io; it can only be yanked.
