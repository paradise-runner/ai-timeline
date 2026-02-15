# AI Timeline — Project Plan

## What It Is

A single-page website that tracks AI company news on a vertical timeline. Scrolling reveals nodes one by one. Each node is color-coded by importance. Tabs at the top let you filter by company or view everything on one unified timeline.

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

## Technical Approach

### Stack
- **HTML** — semantic markup, data attributes for company/importance
- **CSS** — layout, animations, theming, color coding
- **Vanilla JS** — tab filtering only (show/hide nodes by `data-company`), scroll reveal fallback if CSS `animation-timeline` isn't supported enough

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
├── index.html      # single page, all content
├── style.css       # all styling, animations, colors
├── script.js       # tab filtering + scroll reveal fallback
├── data.js         # news entries as JS array (easy to update)
├── PLAN.md         # this file
└── README.md       # setup instructions
```

### Data Model
Each news entry:
```js
{
  date: "2025-01-20",
  company: "openai",
  importance: "critical",    // critical | high | medium | low
  headline: "OpenAI launches o3",
  description: "Next-gen reasoning model..."
}
```

Entries live in `data.js` and get rendered into the DOM. This keeps content separate from structure and makes updates easy — just add a new object to the array.

## Design Decisions

1. **No build tools.** Open `index.html` in a browser and it works.
2. **No frameworks.** This is a content site, not an app. HTML/CSS do the heavy lifting.
3. **Data in JS, not hardcoded HTML.** Makes it maintainable. One array to update.
4. **Mobile-first.** Timeline collapses to single-column on narrow screens, nodes all on one side.
5. **Dark theme default.** Feels right for a tech/AI tracker. Light mode via `prefers-color-scheme`.

## Phases

### Phase 1 — Ship It
- [ ] HTML structure with semantic timeline markup
- [ ] CSS timeline layout with alternating nodes
- [ ] Color system for importance levels
- [ ] Scroll-reveal animations
- [ ] Tab filtering
- [ ] Seed with ~20 real news entries
- [ ] Responsive mobile layout

### Phase 2 — Polish
- [ ] Smooth tab transitions
- [ ] Search/filter by keyword
- [ ] Date range filtering
- [ ] Light/dark mode toggle
- [ ] Permalink to specific nodes

### Phase 3 — Maybe
- [ ] RSS feed integration for auto-updates
- [ ] Contributor submissions
- [ ] Embed links / source URLs on each node
