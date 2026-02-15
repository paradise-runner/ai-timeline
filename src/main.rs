use std::fs;
use std::io::{self, Write as IoWrite};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Entry {
    company_slug: String,
    company_name: String,
    date: String,
    importance: String,
    headline: String,
    description: String,
    url: Option<String>,
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn parse_file(path: &Path) -> Vec<Entry> {
    let slug = path.file_stem().unwrap().to_string_lossy().to_string();
    let content = fs::read_to_string(path).expect("Failed to read file");
    let mut entries = Vec::new();

    let company_name = content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l[2..].trim().to_string())
        .unwrap_or_else(|| slug.clone());

    let parts: Vec<&str> = content.split("\n---\n").collect();

    let mut i = 1;
    while i < parts.len() {
        let header_text = parts[i].trim();
        let mut date = String::new();
        let mut importance = String::new();
        let mut headline = String::new();
        let mut url: Option<String> = None;

        for line in header_text.lines() {
            let line = line.trim();
            if let Some(v) = line.strip_prefix("date:") {
                date = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("importance:") {
                importance = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("headline:") {
                headline = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("url:") {
                let u = v.trim().to_string();
                if !u.is_empty() {
                    url = Some(u);
                }
            }
        }

        let description = if i + 1 < parts.len() {
            parts[i + 1].trim().to_string()
        } else {
            String::new()
        };

        if !date.is_empty() && !headline.is_empty() {
            entries.push(Entry {
                company_slug: slug.clone(),
                company_name: company_name.clone(),
                date,
                importance,
                headline,
                description,
                url,
            });
        }

        i += 2;
    }

    entries
}

fn importance_color(importance: &str) -> &str {
    match importance {
        "inflection" => "#a855f7",
        "critical" => "#ef4444",
        "high" => "#f97316",
        "medium" => "#3b82f6",
        "low" => "#6b7280",
        _ => "#6b7280",
    }
}

fn importance_label(importance: &str) -> &str {
    match importance {
        "inflection" => "Inflection Point",
        "critical" => "Critical",
        "high" => "High Impact",
        "medium" => "Medium",
        "low" => "Low",
        _ => "Unknown",
    }
}

fn format_date_human(date: &str) -> String {
    // Convert "2026-02-12" to "Feb 12, 2026"
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() == 3 {
        let month = match parts[1] {
            "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
            "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
            "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
            _ => parts[1],
        };
        let day = parts[2].trim_start_matches('0');
        format!("{} {}, {}", month, day, parts[0])
    } else {
        date.to_string()
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn generate_html(entries: &[Entry]) -> String {
    let mut companies: Vec<(String, String)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for e in entries {
        if seen.insert(e.company_slug.clone()) {
            companies.push((e.company_slug.clone(), e.company_name.clone()));
        }
    }

    // Collect unique years
    let mut years: Vec<String> = entries
        .iter()
        .filter_map(|e| e.date.split('-').next().map(|y| y.to_string()))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    years.sort();

    let mut tabs_html = String::from(r#"<button class="tab active" data-company="all">All</button>"#);
    for (slug, name) in &companies {
        tabs_html.push_str(&format!(
            r#"<button class="tab" data-company="{slug}">{name}</button>"#
        ));
    }

    let mut year_options = String::new();
    for y in &years {
        year_options.push_str(&format!(r#"<option value="{y}">{y}</option>"#));
    }

    let mut nodes_html = String::new();
    let mut last_year: Option<String> = None;
    for (i, entry) in entries.iter().enumerate() {
        // Year separator markers
        let entry_year = entry.date.split('-').next().unwrap_or("").to_string();
        if last_year.as_ref() != Some(&entry_year) {
            nodes_html.push_str(&format!(
                "<div class=\"year-marker\" data-year=\"{year}\"><span>{year}</span></div>\n",
                year = entry_year
            ));
            last_year = Some(entry_year);
        }

        let side = if i % 2 == 0 { "left" } else { "right" };
        let color = importance_color(&entry.importance);
        let label = importance_label(&entry.importance);
        let node_id = format!("{}-{}-{}", entry.company_slug, entry.date, slugify(&entry.headline));
        let display_date = format_date_human(&entry.date);
        let headline_html = if let Some(ref url) = entry.url {
            format!(
                r#"<a href="{}" target="_blank" rel="noopener" class="timeline-link">{}</a>"#,
                html_escape(url),
                html_escape(&entry.headline)
            )
        } else {
            html_escape(&entry.headline)
        };
        let permalink_href = format!("#{}", node_id);
        nodes_html.push_str(&format!(
            "<div class=\"timeline-item {side}\" id=\"{node_id}\" data-company=\"{slug}\" data-importance=\"{importance}\" data-date=\"{date}\" style=\"--node-color: {color}\">\n\
  <div class=\"timeline-dot\"></div>\n\
  <div class=\"timeline-content\">\n\
    <div class=\"timeline-meta\">\n\
      <span class=\"timeline-date\">{display_date}</span>\n\
      <span class=\"timeline-badge\" style=\"background: {color}\">{label}</span>\n\
      <span class=\"timeline-company\">{company}</span>\n\
      <a href=\"{permalink}\" class=\"permalink\" title=\"Link to this event\">#</a>\n\
    </div>\n\
    <h3 class=\"timeline-headline\">{headline_html}</h3>\n\
    <p class=\"timeline-desc\">{desc}</p>\n\
  </div>\n\
</div>\n",
            side = side,
            node_id = node_id,
            slug = entry.company_slug,
            importance = entry.importance,
            color = color,
            date = entry.date,
            display_date = display_date,
            label = label,
            company = entry.company_name,
            permalink = permalink_href,
            headline_html = headline_html,
            desc = html_escape(&entry.description),
        ));
    }

    format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>AI Timeline</title>
<style>
* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
  background: #0a0a0f;
  color: #e4e4e7;
  line-height: 1.6;
  min-height: 100vh;
  transition: background 0.3s, color 0.3s;
}}
/* Light mode */
body.light {{
  background: #f8f8fc;
  color: #1a1a2e;
}}
body.light .tab-bar {{
  background: #f8f8fcee;
  border-bottom-color: #d4d4d8;
}}
body.light .tab {{
  background: #e4e4e7;
  border-color: #d4d4d8;
  color: #52525b;
}}
body.light .tab:hover {{ background: #d4d4d8; color: #1a1a2e; }}
body.light .tab.active {{ background: #3b82f6; border-color: #3b82f6; color: #fff; }}
body.light .filter {{
  background: #e4e4e7;
  border-color: #d4d4d8;
  color: #52525b;
}}
body.light .filter:hover {{ background: #d4d4d8; color: #1a1a2e; }}
body.light .filter.active {{ background: #a855f7; border-color: #a855f7; color: #fff; }}
body.light .timeline::before {{ background: #d4d4d8; }}
body.light .timeline-content {{
  background: #fff;
  border-color: #d4d4d8;
}}
body.light .timeline-headline {{ color: #1a1a2e; }}
body.light .timeline-desc {{ color: #52525b; }}
body.light .timeline-date {{ color: #71717a; }}
body.light .timeline-company {{ color: #71717a; }}
body.light .timeline-dot {{ border-color: #f8f8fc; }}
body.light .header p {{ color: #71717a; }}
body.light .controls-bar {{ background: #f8f8fcee; border-bottom-color: #d4d4d8; }}
body.light .search-input {{
  background: #e4e4e7;
  border-color: #d4d4d8;
  color: #1a1a2e;
}}
body.light .search-input::placeholder {{ color: #a1a1aa; }}
body.light .year-select {{
  background: #e4e4e7;
  border-color: #d4d4d8;
  color: #1a1a2e;
}}
body.light .theme-toggle {{
  background: #e4e4e7;
  border-color: #d4d4d8;
  color: #52525b;
}}
.tab-bar {{
  position: sticky;
  top: 0;
  z-index: 100;
  background: #0a0a0fee;
  backdrop-filter: blur(12px);
  padding: 16px 20px 8px;
  display: flex;
  gap: 8px;
  justify-content: center;
  flex-wrap: wrap;
  border-bottom: 1px solid #27272a;
}}
.controls-bar {{
  position: sticky;
  top: 0;
  z-index: 99;
  background: #0a0a0fee;
  backdrop-filter: blur(12px);
  padding: 8px 20px;
  display: flex;
  gap: 10px;
  justify-content: center;
  align-items: center;
  flex-wrap: wrap;
  border-bottom: 1px solid #27272a;
}}
.search-input {{
  background: #18181b;
  border: 1px solid #27272a;
  color: #e4e4e7;
  padding: 6px 14px;
  border-radius: 9999px;
  font-size: 13px;
  width: 200px;
  outline: none;
  transition: border-color 0.2s;
}}
.search-input:focus {{ border-color: #3b82f6; }}
.search-input::placeholder {{ color: #52525b; }}
.year-select {{
  background: #18181b;
  border: 1px solid #27272a;
  color: #a1a1aa;
  padding: 6px 12px;
  border-radius: 9999px;
  font-size: 13px;
  cursor: pointer;
  outline: none;
}}
.theme-toggle {{
  background: #18181b;
  border: 1px solid #27272a;
  color: #a1a1aa;
  padding: 6px 12px;
  border-radius: 9999px;
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
  transition: all 0.2s;
}}
.theme-toggle:hover {{ background: #27272a; color: #e4e4e7; }}
.tab {{
  background: #18181b;
  border: 1px solid #27272a;
  color: #a1a1aa;
  padding: 8px 18px;
  border-radius: 9999px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}}
.tab:hover {{ background: #27272a; color: #e4e4e7; }}
.tab.active {{ background: #3b82f6; border-color: #3b82f6; color: #fff; }}
.filter {{
  background: #18181b;
  border: 1px solid #27272a;
  color: #a1a1aa;
  padding: 6px 14px;
  border-radius: 9999px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
}}
.filter:hover {{ background: #27272a; color: #e4e4e7; }}
.filter.active {{ background: #a855f7; border-color: #a855f7; color: #fff; }}
.header {{
  text-align: center;
  padding: 60px 20px 40px;
}}
.header h1 {{
  font-size: 2.5rem;
  font-weight: 700;
  background: linear-gradient(135deg, #3b82f6, #a855f7);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}}
.header p {{
  color: #71717a;
  margin-top: 8px;
  font-size: 1.1rem;
}}
.timeline {{
  position: relative;
  max-width: 900px;
  margin: 0 auto;
  padding: 20px 20px 80px;
}}
.timeline::before {{
  content: '';
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 2px;
  background: #27272a;
  transform: translateX(-50%);
}}
.timeline-item {{
  position: relative;
  width: 50%;
  padding: 10px 40px 30px;
  opacity: 0;
  transform: translateY(30px);
  transition: opacity 0.6s ease, transform 0.6s ease, max-height 0.4s ease;
  max-height: 1000px;
  overflow: hidden;
}}
.timeline-item.visible {{
  opacity: 1;
  transform: translateY(0);
}}
.timeline-item.left {{
  left: 0;
  text-align: right;
}}
.timeline-item.right {{
  left: 50%;
  text-align: left;
}}
.timeline-item.hidden {{
  opacity: 0;
  max-height: 0;
  padding-top: 0;
  padding-bottom: 0;
  margin: 0;
  overflow: hidden;
  pointer-events: none;
  transform: translateY(-10px);
}}
.timeline-dot {{
  position: absolute;
  top: 18px;
  width: 16px;
  height: 16px;
  background: var(--node-color);
  border: 3px solid #0a0a0f;
  border-radius: 50%;
  z-index: 2;
  box-shadow: 0 0 10px var(--node-color);
}}
.timeline-item.left .timeline-dot {{
  right: -8px;
}}
.timeline-item.right .timeline-dot {{
  left: -8px;
}}
.timeline-content {{
  background: #18181b;
  border: 1px solid #27272a;
  border-radius: 12px;
  padding: 20px;
  transition: border-color 0.3s;
  position: relative;
}}
.timeline-content:hover {{
  border-color: var(--node-color);
}}
.timeline-meta {{
  display: flex;
  gap: 8px;
  align-items: center;
  margin-bottom: 8px;
  flex-wrap: wrap;
}}
.timeline-item.left .timeline-meta {{
  justify-content: flex-end;
}}
.timeline-date {{
  color: #71717a;
  font-size: 13px;
  font-weight: 500;
}}
.timeline-badge {{
  font-size: 11px;
  font-weight: 600;
  padding: 2px 10px;
  border-radius: 9999px;
  color: #fff;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}}
.timeline-company {{
  font-size: 12px;
  color: #a1a1aa;
  font-weight: 500;
}}
.timeline-headline {{
  font-size: 1.1rem;
  font-weight: 600;
  color: #f4f4f5;
  margin-bottom: 6px;
}}
.timeline-link {{
  color: inherit;
  text-decoration: none;
  border-bottom: 1px dashed currentColor;
  transition: opacity 0.2s;
}}
.timeline-link:hover {{ opacity: 0.8; }}
.timeline-desc {{
  color: #a1a1aa;
  font-size: 14px;
  line-height: 1.5;
}}
.permalink {{
  color: #52525b;
  text-decoration: none;
  font-size: 14px;
  font-weight: 700;
  opacity: 0;
  transition: opacity 0.2s;
  margin-left: auto;
}}
.timeline-content:hover .permalink {{
  opacity: 1;
}}
.permalink:hover {{ color: #3b82f6; }}

.year-marker {{
  position: relative;
  text-align: center;
  margin: 20px 0;
  z-index: 3;
  clear: both;
  width: 100%;
}}
.year-marker span {{
  display: inline-block;
  background: linear-gradient(135deg, #3b82f6, #a855f7);
  color: #fff;
  font-weight: 700;
  font-size: 15px;
  padding: 6px 24px;
  border-radius: 9999px;
  letter-spacing: 1px;
  box-shadow: 0 2px 12px rgba(99,102,241,0.3);
  position: relative;
  left: 50%;
  transform: translateX(-50%);
}}
body.light .year-marker span {{
  box-shadow: 0 2px 12px rgba(99,102,241,0.2);
}}
.back-to-top {{
  position: fixed;
  bottom: 32px;
  right: 32px;
  z-index: 200;
  background: linear-gradient(135deg, #3b82f6, #a855f7);
  color: #fff;
  border: none;
  border-radius: 50%;
  width: 48px;
  height: 48px;
  font-size: 22px;
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(0,0,0,0.3);
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.3s, transform 0.3s;
  transform: translateY(10px);
}}
.back-to-top.show {{
  opacity: 1;
  pointer-events: auto;
  transform: translateY(0);
}}
.back-to-top:hover {{ transform: scale(1.1); }}
.importance-label {{
  color: #a1a1aa;
  font-size: 13px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}}
body.light .importance-label {{ color: #71717a; }}
@media (max-width: 700px) {{
  .timeline::before {{ left: 20px; }}
  .timeline-item {{
    width: 100%;
    left: 0 !important;
    text-align: left !important;
    padding-left: 50px;
    padding-right: 16px;
  }}
  .timeline-item .timeline-meta {{
    justify-content: flex-start !important;
  }}
  .timeline-dot {{
    left: 12px !important;
    right: auto !important;
  }}
  .header h1 {{ font-size: 1.8rem; }}
  .search-input {{ width: 140px; }}
}}
</style>
</head>
<body>
<div class="header">
  <h1>AI Timeline</h1>
  <p>Tracking the AI landscape, one event at a time</p>
</div>
<div class="tab-bar">
  {tabs}
</div>
<div class="controls-bar">
  <input type="text" class="search-input" placeholder="Search..." id="searchInput">
  <select class="year-select" id="yearFrom"><option value="">From</option>{year_options}</select>
  <select class="year-select" id="yearTo"><option value="">To</option>{year_options_rev}</select>
  <span class="importance-label">Importance:</span>
  <button class="filter active" data-min="all">All</button>
  <button class="filter" data-min="medium">Medium+</button>
  <button class="filter" data-min="high">High+</button>
  <button class="filter" data-min="critical">Critical+</button>
  <button class="filter" data-min="inflection">Inflection</button>
  <button class="theme-toggle" id="themeToggle" title="Toggle light/dark mode">🌙</button>
</div>
<div class="timeline">
  {nodes}
</div>
<button class="back-to-top" id="backToTop" title="Back to top">↑</button>
<script>
const levels = ['low', 'medium', 'high', 'critical', 'inflection'];
let activeCompany = 'all';
let activeMin = 'all';
let searchText = '';
let yearFrom = '';
let yearTo = '';

function applyFilters() {{
  const minIdx = activeMin === 'all' ? 0 : levels.indexOf(activeMin);
  const q = searchText.toLowerCase();
  document.querySelectorAll('.timeline-item').forEach(item => {{
    const companyMatch = activeCompany === 'all' || item.dataset.company === activeCompany;
    const impIdx = levels.indexOf(item.dataset.importance);
    const impMatch = impIdx >= minIdx;
    const dateYear = item.dataset.date.substring(0, 4);
    const yearMatch = (!yearFrom || dateYear >= yearFrom) && (!yearTo || dateYear <= yearTo);
    const text = item.textContent.toLowerCase();
    const searchMatch = !q || text.includes(q);
    if (companyMatch && impMatch && yearMatch && searchMatch) {{
      item.classList.remove('hidden');
    }} else {{
      item.classList.add('hidden');
    }}
  }});
  observe();
}}

document.querySelectorAll('.tab').forEach(tab => {{
  tab.addEventListener('click', () => {{
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    tab.classList.add('active');
    activeCompany = tab.dataset.company;
    applyFilters();
  }});
}});

document.querySelectorAll('.filter').forEach(btn => {{
  btn.addEventListener('click', () => {{
    document.querySelectorAll('.filter').forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    activeMin = btn.dataset.min;
    applyFilters();
  }});
}});

document.getElementById('searchInput').addEventListener('input', e => {{
  searchText = e.target.value;
  applyFilters();
}});

document.getElementById('yearFrom').addEventListener('change', e => {{
  yearFrom = e.target.value;
  applyFilters();
}});

document.getElementById('yearTo').addEventListener('change', e => {{
  yearTo = e.target.value;
  applyFilters();
}});

// Theme toggle
const toggle = document.getElementById('themeToggle');
const saved = localStorage.getItem('theme');
if (saved === 'light') {{ document.body.classList.add('light'); toggle.textContent = '☀️'; }}
toggle.addEventListener('click', () => {{
  document.body.classList.toggle('light');
  const isLight = document.body.classList.contains('light');
  toggle.textContent = isLight ? '☀️' : '🌙';
  localStorage.setItem('theme', isLight ? 'light' : 'dark');
}});

// Permalink click
document.querySelectorAll('.permalink').forEach(a => {{
  a.addEventListener('click', e => {{
    if (navigator.clipboard) {{
      navigator.clipboard.writeText(a.href);
    }}
  }});
}});

// Scroll to hash on load
if (location.hash) {{
  const el = document.querySelector(location.hash);
  if (el) {{ el.classList.add('visible'); setTimeout(() => el.scrollIntoView({{ behavior: 'smooth', block: 'center' }}), 100); }}
}}

// Single global observer
const observer = new IntersectionObserver((entries) => {{
  entries.forEach(e => {{
    if (e.isIntersecting) {{
      e.target.classList.add('visible');
      observer.unobserve(e.target);
    }}
  }});
}}, {{ threshold: 0.05, rootMargin: '0px 0px 50px 0px' }});

function observe() {{
  document.querySelectorAll('.timeline-item:not(.hidden):not(.visible)').forEach(item => {{
    observer.observe(item);
  }});
}}
observe();

// Back to top
const backBtn = document.getElementById('backToTop');
window.addEventListener('scroll', () => {{
  backBtn.classList.toggle('show', window.scrollY > 400);
}});
backBtn.addEventListener('click', () => {{
  window.scrollTo({{ top: 0, behavior: 'smooth' }});
}});
</script>
</body>
</html>"##,
        tabs = tabs_html,
        nodes = nodes_html,
        year_options = year_options,
        year_options_rev = {
            let mut rev = years.clone();
            rev.reverse();
            rev.iter().map(|y| format!(r#"<option value="{y}">{y}</option>"#)).collect::<String>()
        },
    )
}

fn build(news_dir: &Path, out_dir: &Path) {
    let mut all_entries: Vec<Entry> = Vec::new();

    if news_dir.is_dir() {
        let mut files: Vec<PathBuf> = fs::read_dir(news_dir)
            .expect("Cannot read news/")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "md").unwrap_or(false))
            .collect();
        files.sort();

        for file in files {
            let entries = parse_file(&file);
            all_entries.extend(entries);
        }
    }

    all_entries.sort_by(|a, b| b.date.cmp(&a.date));

    let html = generate_html(&all_entries);
    fs::create_dir_all(out_dir).expect("Cannot create dist/");
    fs::write(out_dir.join("index.html"), html).expect("Cannot write index.html");
    println!("Built {} entries → {}/index.html", all_entries.len(), out_dir.display());
}

fn new_entry(company: &str, news_dir: &Path) {
    let file_path = news_dir.join(format!("{}.md", company));
    if !file_path.exists() {
        eprintln!("File not found: {}. Creating new file.", file_path.display());
        fs::write(&file_path, format!("# {}\n", company.chars().next().unwrap().to_uppercase().to_string() + &company[1..])).ok();
    }

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    print!("Date [{}]: ", today);
    io::stdout().flush().ok();
    let mut date = String::new();
    io::stdin().read_line(&mut date).ok();
    let date = date.trim();
    let date = if date.is_empty() { today } else { date.to_string() };

    print!("Importance (low/medium/high/critical/inflection) [medium]: ");
    io::stdout().flush().ok();
    let mut importance = String::new();
    io::stdin().read_line(&mut importance).ok();
    let importance = importance.trim();
    let importance = if importance.is_empty() { "medium" } else { importance };

    print!("Headline: ");
    io::stdout().flush().ok();
    let mut headline = String::new();
    io::stdin().read_line(&mut headline).ok();
    let headline = headline.trim();
    if headline.is_empty() {
        eprintln!("Headline is required.");
        std::process::exit(1);
    }

    print!("URL (optional): ");
    io::stdout().flush().ok();
    let mut url = String::new();
    io::stdin().read_line(&mut url).ok();
    let url = url.trim();

    let mut block = format!("\n---\ndate: {}\nimportance: {}\nheadline: {}\n", date, importance, headline);
    if !url.is_empty() {
        block.push_str(&format!("url: {}\n", url));
    }
    block.push_str("---\n\nTODO: Add description here.\n");

    let mut file = fs::OpenOptions::new().append(true).open(&file_path).expect("Cannot open file");
    file.write_all(block.as_bytes()).expect("Cannot write to file");
    println!("Added entry to {}", file_path.display());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("build");

    let news_dir = PathBuf::from("news");
    let out_dir = PathBuf::from("dist");

    match command {
        "build" => build(&news_dir, &out_dir),
        "serve" => {
            build(&news_dir, &out_dir);
            println!("Serving on http://localhost:3000");
            let server = tiny_http::Server::http("0.0.0.0:3000").expect("Cannot start server");
            for request in server.incoming_requests() {
                let html = fs::read_to_string(out_dir.join("index.html")).unwrap_or_default();
                let response = tiny_http::Response::from_string(html)
                    .with_header("Content-Type: text/html; charset=utf-8".parse::<tiny_http::Header>().unwrap());
                let _ = request.respond(response);
            }
        }
        "new" => {
            let company = args.get(2).map(|s| s.as_str()).unwrap_or_else(|| {
                eprintln!("Usage: ai-timeline new <company>");
                std::process::exit(1);
            });
            new_entry(company, &news_dir);
        }
        _ => {
            eprintln!("Usage: ai-timeline [build|serve|new <company>]");
            std::process::exit(1);
        }
    }
}
