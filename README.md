# AI Timeline

A visual, interactive timeline tracking the AI landscape — major model launches, acquisitions, regulations, and milestones from OpenAI, Anthropic, Google, Meta, xAI, Mistral, and more.

![Screenshot](screenshot.png)

## Features

- Vertical timeline with color-coded importance levels (🟣 Inflection → ⚪ Low)
- Filter by company, importance level, keyword search, and date range
- Light/dark mode with localStorage persistence
- Permalink anchors for sharing specific events
- Scroll-reveal animations
- Responsive mobile layout
- Single static HTML output — no runtime dependencies

## Setup

```bash
# Clone and build
git clone https://github.com/yourusername/ai-timeline.git
cd ai-timeline
cargo build

# Generate the site
cargo run -- build

# Preview locally
cargo run -- serve
# → http://localhost:3000
```

## Adding News Entries

Each company has a markdown file in `news/`:

```
news/openai.md
news/anthropic.md
news/google.md
...
```

### Quick add via CLI

```bash
cargo run -- new openai
# Prompts for date, importance, headline, URL
```

### Manual format

```markdown
---
date: 2025-01-20
importance: critical
headline: OpenAI launches o3
url: https://openai.com/blog/o3
---

Description of the event goes here.
```

Fields: `date` (YYYY-MM-DD), `importance` (low/medium/high/critical/inflection), `headline`, `url` (optional).

## Importance Levels

| Color | Level | Use for |
|-------|-------|---------|
| 🟣 Purple | inflection | Paradigm shifts, existential milestones |
| 🔴 Red | critical | New model launches, major acquisitions |
| 🟠 Orange | high | Significant releases, big funding |
| 🔵 Blue | medium | Notable updates, leadership changes |
| ⚪ Gray | low | Minor updates, rumors |

## Deployment

The site builds to `dist/index.html` — a single self-contained HTML file. Deploy anywhere:

- **GitHub Pages**: Push to `main` and enable Pages on `dist/` (or use the included GitHub Actions workflow)
- **Netlify/Vercel**: Set build command to `cargo run -- build` and publish directory to `dist`
- **Manual**: Just copy `dist/index.html` to any web server

## License

MIT
