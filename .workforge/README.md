# WorkForge workspace

This directory contains the planning workspace for this repository. Requirements
live in `inbox/`; generated provider state and agent context live in `output/`.

## Contributor workflow

1. Copy `.env.example` to `.env` and add your provider credentials.
2. Validate access with `uvx workforge providers test --workspace .workforge`.

Run WorkForge through `uvx workforge` so contributors use an isolated CLI
without installing it into the project.

### Start from a requirement

Create `inbox/<requirement>.md` with one `## Title` section per planning item,
optional metadata such as `Labels: docs`, a description, and `- Task` lines.
Then preview it before any external write:

`uvx workforge preview .workforge/inbox/<requirement>.md --workspace .workforge`

After approval, create and save the provider items:

`uvx workforge create .workforge/inbox/<requirement>.md --workspace .workforge --execute --save`

### Continue existing items

Discover existing provider items into the default `discovered` output:

`uvx workforge discover --workspace .workforge --save`

Claim the selected item and move it into progress:

`uvx workforge claim-item discovered --workspace .workforge --item <id> --assignee @me`

`uvx workforge move-item discovered --workspace .workforge --item <id> --status doing`

Continue using `discovered` with `status` and `complete-task` to keep the issue
checklist synchronized.

Never commit `.env` or `output/`. See the repository's contributor and agent
guides for its development, validation, and pull request rules. See the
[WorkForge README](https://github.com/Nanielito/workforge#quick-start) for `uv`
installation, provider configuration, and the complete command reference.
