use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Entry {
    company_slug: String,
    company_name: String,
    date: String,
    importance: String,
    headline: String,
    description: String,
}

fn parse_file(path: &Path) -> Vec<Entry> {
    let slug = path.file_stem().unwrap().to_string_lossy().to_string();
    let content = fs::read_to_string(path).expect("Failed to read file");
    let mut entries = Vec::new();

    // Extract company name from first # heading
    let company_name = content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l[2..].trim().to_string())
        .unwrap_or_else(|| slug.clone());

    // Split on --- but we need to find entry blocks
    // Pattern: ---\ndate: ...\nimportance: ...\nheadline: ...\n---\n\nDescription
    let parts: Vec<&str> = content.split("\n---\n").collect();
    // parts[0] = "# Company\n" (skip)
    // parts[1] = "date: ...\nimportance: ...\nheadline: ...\n" (header)
    // parts[2] = "Description\n" OR "Description\n\n" followed by next header
    // Actually the pattern alternates: odd indices are headers, the text after until next --- is description

    let mut i = 1; // skip the first part (company heading)
    while i < parts.len() {
        let header_text = parts[i].trim();
        // Parse header fields
        let mut date = String::new();
        let mut importance = String::new();
        let mut headline = String::new();

        for line in header_text.lines() {
            let line = line.trim();
            if let Some(v) = line.strip_prefix("date:") {
                date = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("importance:") {
                importance = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("headline:") {
                headline = v.trim().to_string();
            }
        }

        // Description is next part
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
            });
        }

        i += 2; // skip to next header
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

fn generate_html(entries: &[Entry]) -> String {
    // Collect unique companies in order
    let mut companies: Vec<(String, String)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for e in entries {
        if seen.insert(e.company_slug.clone()) {
            companies.push((e.company_slug.clone(), e.company_name.clone()));
        }
    }

    let mut tabs_html = String::from(r#"<button class="tab active" data-company="all">All</button>"#);
    for (slug, name) in &companies {
        tabs_html.push_str(&format!(
            r#"<button class="tab" data-company="{slug}">{name}</button>"#
        ));
    }

    let mut nodes_html = String::new();
    for (i, entry) in entries.iter().enumerate() {
        let side = if i % 2 == 0 { "left" } else { "right" };
        let color = importance_color(&entry.importance);
        let label = importance_label(&entry.importance);
        nodes_html.push_str(&format!(
            r#"<div class="timeline-item {side}" data-company="{slug}" data-importance="{importance}" style="--node-color: {color}">
  <div class="timeline-dot"></div>
  <div class="timeline-content">
    <div class="timeline-meta">
      <span class="timeline-date">{date}</span>
      <span class="timeline-badge" style="background: {color}">{label}</span>
      <span class="timeline-company">{company}</span>
    </div>
    <h3 class="timeline-headline">{headline}</h3>
    <p class="timeline-desc">{desc}</p>
  </div>
</div>
"#,
            side = side,
            slug = entry.company_slug,
            importance = entry.importance,
            color = color,
            date = entry.date,
            label = label,
            company = entry.company_name,
            headline = entry.headline,
            desc = entry.description,
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
}}
.tab-bar {{
  position: sticky;
  top: 0;
  z-index: 100;
  background: #0a0a0fee;
  backdrop-filter: blur(12px);
  padding: 16px 20px;
  display: flex;
  gap: 8px;
  justify-content: center;
  flex-wrap: wrap;
  border-bottom: 1px solid #27272a;
}}
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
.filter-bar {{
  display: flex;
  justify-content: center;
  gap: 8px;
  padding: 0 20px 30px;
  flex-wrap: wrap;
}}
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
  transition: opacity 0.6s ease, transform 0.6s ease;
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
.timeline-item.hidden {{ display: none; }}
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
.timeline-desc {{
  color: #a1a1aa;
  font-size: 14px;
  line-height: 1.5;
}}

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
<div class="filter-bar">
  <button class="filter active" data-min="all">All</button>
  <button class="filter" data-min="low">Low+</button>
  <button class="filter" data-min="medium">Medium+</button>
  <button class="filter" data-min="high">High+</button>
  <button class="filter" data-min="critical">Critical+</button>
  <button class="filter" data-min="inflection">Inflection</button>
</div>
<div class="timeline">
  {nodes}
</div>
<script>
const levels = ['low', 'medium', 'high', 'critical', 'inflection'];
let activeCompany = 'all';
let activeMin = 'all';

function applyFilters() {{
  const minIdx = activeMin === 'all' ? 0 : levels.indexOf(activeMin);
  document.querySelectorAll('.timeline-item').forEach(item => {{
    const companyMatch = activeCompany === 'all' || item.dataset.company === activeCompany;
    const impIdx = levels.indexOf(item.dataset.importance);
    const impMatch = impIdx >= minIdx;
    if (companyMatch && impMatch) {{
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

function observe() {{
  const items = document.querySelectorAll('.timeline-item:not(.hidden)');
  const observer = new IntersectionObserver((entries) => {{
    entries.forEach(e => {{
      if (e.isIntersecting) {{
        e.target.classList.add('visible');
      }}
    }});
  }}, {{ threshold: 0.1 }});
  items.forEach(item => observer.observe(item));
}}
observe();
</script>
</body>
</html>"##, tabs = tabs_html, nodes = nodes_html)
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

    // Sort by date descending
    all_entries.sort_by(|a, b| b.date.cmp(&a.date));

    let html = generate_html(&all_entries);
    fs::create_dir_all(out_dir).expect("Cannot create dist/");
    fs::write(out_dir.join("index.html"), html).expect("Cannot write index.html");
    println!("Built {} entries → {}/index.html", all_entries.len(), out_dir.display());
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
        _ => {
            eprintln!("Usage: ai-timeline [build|serve]");
            std::process::exit(1);
        }
    }
}
