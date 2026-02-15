# AI Timeline — Project Plan

## What It Is

A single-page website that tracks AI company news on a vertical timeline. Scrolling reveals nodes one by one. Each node is color-coded by importance. Tabs at the top let you filter by company or view everything on one unified timeline.

A Rust CLI reads markdown files from `news/` and generates the static site.

## Who It's For

Anyone following the AI landscape who wants a visual, chronological way to see what's happening — without doomscrolling Twitter or reading 15 newsletters.

## Core Experience

### The Timeline
- Vertical line running down the center of the page
- News nodes alternate left/right along the line
- Each node contains: date, headline, short description, company tag
- Nodes fade/slide in as you scroll down (CSS-only scroll-driven animations)
- Most recent news at top, oldest at bottom

### Node Colors (Importance)
| Color | Level | Meaning |
|-------|-------|---------|
| 🟣 Purple | Inflection | Possible industry inflection point — paradigm shift, existential risk milestone, regulatory sea change |
| 🔴 Red | Critical | Industry-shifting (new model launch, major acquisition, regulation) |
| 🟠 Orange | High | Significant product release, major partnership, big funding round |
| 🔵 Blue | Medium | Notable update, feature launch, leadership change |
| ⚪ Gray | Low | Minor update, rumor, incremental improvement |

### Company Tabs
- Fixed tab bar at the top of the page
- Tabs: **All** | **OpenAI** | **Anthropic** | **Google** | **Meta** | **xAI** | **Mistral** | **Other**
- "All" shows a unified timeline with all companies interleaved chronologically
- Company tabs show only that company's nodes
- Active tab is visually highlighted
- Tab switching uses minimal vanilla JS (toggle CSS classes)

## Content — Markdown News Files

Each company has its own markdown file in the `news/` directory:

```
news/
├── openai.md
├── anthropic.md
├── google.md
├── meta.md
├── xai.md
├── mistral.md
└── other.md
```

### Markdown Format

Each file contains entries separated by `---`. Frontmatter-style metadata per entry:

```markdown
# OpenAI

---
date: 2025-01-20
importance: critical
headline: OpenAI launches o3
---

Next-generation reasoning model with significant improvements in math, coding, and scientific benchmarks. Available to Plus and Team subscribers.

---
date: 2025-01-10
importance: high
headline: ChatGPT hits 300M weekly users
---

OpenAI reports record usage numbers, driven by o1 launch and enterprise adoption.
```

Rules:
- One file per company, named with the company slug
- Entries are separated by `---`
- Each entry starts with a YAML-like header block between `---` markers: `date`, `importance`, `headline`
- Body text after the header is the description (supports markdown)
- Company is derived from the filename, not repeated per entry
- The `# Title` at the top of each file is the display name for tabs

## Technical Approach

### Stack
- **Rust CLI** — reads `news/*.md`, parses entries, generates static HTML
- **HTML** — semantic markup, data attributes for company/importance
- **CSS** — layout, animations, theming, color coding
- **Vanilla JS** — tab filtering + scroll reveal fallback

### Rust CLI (`ai-timeline`)

The CLI is the build tool. It:

1. Scans `news/` for `.md` files
2. Parses each file — extracts the company name from `# Title`, splits entries on `---`, parses date/importance/headline/description
3. Merges all entries, sorts by date descending
4. Renders `dist/index.html` using an embedded HTML template with inlined CSS and JS
5. Copies any static assets to `dist/`

```
Usage:
  ai-timeline build              # generate dist/index.html from news/*.md
  ai-timeline build --out <dir>  # specify output directory
  ai-timeline serve              # build + serve locally on :3000
  ai-timeline new <company>      # scaffold a new entry in news/<company>.md
```

Key crates:
- `pulldown-cmark` or manual parsing for the markdown/frontmatter split
- `askama` or `minijinja` for HTML templating
- Basic file I/O — no web framework needed for build mode
- `tiny_http` or similar for the `serve` command

### CSS Scroll-Driven Animations
- Use `animation-timeline: view()` where supported
- Nodes start with `opacity: 0; transform: translateX(-30px)` (or `+30px` for right side)
- Animate to full opacity and position as they enter viewport
- Fallback: `IntersectionObserver` in ~10 lines of JS

### Layout
```
┌─────────────────────────────────┐
│  [All] [OpenAI] [Anthropic] ... │  ← sticky tab bar
├─────────────────────────────────┤
│                                 │
│    ┌──────┐    │                │
│    │ Node │────●                │  ← left node
│    └──────┘    │                │
│                │    ┌──────┐    │
│                ●────│ Node │    │  ← right node
│                │    └──────┘    │
│                │                │
│    ┌──────┐    │                │
│    │ Node │────●                │
│    └──────┘    │                │
│                                 │
└─────────────────────────────────┘
```

### File Structure
```
ai-timeline/
├── news/                # content lives here
│   ├── openai.md
│   ├── anthropic.md
│   ├── google.md
│   ├── meta.md
│   ├── xai.md
│   ├── mistral.md
│   └── other.md
├── templates/           # HTML template(s) for the CLI
│   └── index.html
├── static/              # CSS + JS (embedded into output by CLI)
│   ├── style.css
│   └── script.js
├── src/                 # Rust source
│   └── main.rs
├── dist/                # generated output (gitignored)
│   └── index.html
├── Cargo.toml
├── PLAN.md
└── README.md
```

## Design Decisions

1. **Markdown for content.** Easy to author, diff, and review in PRs. No database.
2. **One file per company.** Keeps things organized. Company name derived from file — no repetition.
3. **Rust CLI generates static HTML.** Fast, single binary, no runtime deps. Just run `ai-timeline build`.
4. **No frameworks.** The output is a single HTML file with inlined CSS/JS. Open it in a browser.
5. **Mobile-first.** Timeline collapses to single-column on narrow screens, nodes all on one side.
6. **Dark theme default.** Feels right for a tech/AI tracker. Light mode via `prefers-color-scheme`.

## Phases

### Phase 1 — Ship It ✅
- [x] Rust CLI: parse markdown files, generate HTML
- [x] HTML template with semantic timeline markup
- [x] CSS timeline layout with alternating nodes
- [x] Color system for importance levels (purple/red/orange/blue/gray)
- [x] Scroll-reveal animations (IntersectionObserver)
- [x] Company tab filtering (vanilla JS)
- [x] Importance level filtering (All / Medium+ / High+ / Critical+ / Inflection)
- [x] 140 news entries across 7 companies, balanced 2015–2026
- [x] Responsive mobile layout
- [x] `serve` command for local preview
- [x] GitHub Actions: build + deploy to Pages on push
- [x] Dark theme

### Phase 2 — Polish ✅
- [x] `new` command to scaffold entries with date/importance/headline prompts
- [x] Smooth tab/filter transitions (animate node show/hide)
- [x] Search/filter by keyword
- [x] Date range filtering (year dropdowns)
- [x] Light/dark mode toggle (localStorage persistence)
- [x] Permalink to specific nodes (anchor links with copy on hover)
- [x] Source URLs on each node (optional `url:` field in entry header)
- [x] README with setup/usage instructions
