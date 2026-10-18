# Sonvik crates.io Trusted Publishing

This requirement replaces the temporary crates.io API token used for the first release with short-lived credentials issued through GitHub Actions OIDC. It changes release authentication only; release-plz remains responsible for versioning, tags, and crates.io publication.

## Migrate crates.io publication to Trusted Publishing

Source: product_requirements
Priority: high
Milestone: post-v0.1.0
Labels: ci, maintenance

Configure the already-published `sonvik` crate to trust the existing release-plz publication job, then remove and revoke the temporary registry token without changing the release approval flow.

- Register a GitHub Actions trusted publisher for `sonvik` on crates.io with owner `Nanielito`, repository `sonvik`, and workflow `release-plz.yml`
- Grant `id-token: write` only to the `release` job in `.github/workflows/release-plz.yml`
- Remove `CARGO_REGISTRY_TOKEN` from the release-plz workflow; do not add a separate authentication action because release-plz obtains the short-lived crates.io token directly
- Keep the `release-pr` job without OIDC permission and preserve the existing human-reviewed release PR gate
- Update the synchronized Spanish and English release documentation to describe Trusted Publishing as the active mechanism and the exact one-time crates.io configuration
- Verify the workflow syntax and confirm a release-plz run can authenticate through OIDC without reading `CARGO_REGISTRY_TOKEN`
- After the OIDC path is verified, revoke the temporary token on crates.io and delete the `CARGO_REGISTRY_TOKEN` GitHub Actions secret
- Record manual revocation and secret deletion as manual checks; do not infer their completion from CI
- Verify ordinary pushes and unmerged release pull requests still cannot publish a crate or create a version tag
