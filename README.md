# AI Timeline

A visual timeline tracking major AI industry events. Built with Rust, deployed as a single static HTML page.

## Usage

```bash
# Build the site
cargo run -- build

# Build and serve locally
cargo run -- serve
```

## Adding News

Add entries to the appropriate file in `news/`. Format:

```markdown
---
date: 2025-01-20
importance: critical
headline: Something happened
---

Description of what happened.
```

Importance levels: `inflection` (purple), `critical` (red), `high` (orange), `medium` (blue), `low` (gray)

## Deployment

Pushes to `main` automatically deploy to GitHub Pages.
