# Sonvik documentation site

## Create the MkDocs documentation site

Source: product_requirements
Priority: medium
Milestone: v0.1.0
Labels: docs

Create a simple public-facing site that introduces Sonvik before directing readers to the existing documentation. Keep Markdown as the source of truth and use only MkDocs' built-in capabilities.

- Add a minimal MkDocs configuration with the built-in theme and search
- Create a concise landing page from the main README's project presentation
- Link the landing page to the Spanish, English, and architecture decision documentation
- Organize navigation with Spanish first as the canonical language
- Preserve the existing documentation content and directory structure
- Pin MkDocs to the current 1.x release line for reproducible builds
- Verify the site builds successfully with strict validation

## Publish the documentation with GitHub Pages

Source: product_requirements
Priority: medium
Milestone: v0.1.0
Labels: ci, docs

Publish the MkDocs site automatically on GitHub Pages without paid services or manually committed build artifacts.

- Add a GitHub Actions workflow that builds the site on changes to the default branch
- Deploy the generated static site through GitHub Pages' official actions
- Grant the workflow only the permissions required for Pages deployment
- Keep generated site files out of source control
- Add the public documentation URL to the repository README
- Verify the deployed landing page and documentation links work
