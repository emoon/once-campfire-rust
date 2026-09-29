import re, subprocess, sys
S = sys.argv[1]
stages = ["message_presentation", "to_plain_text", "render_nested", "sanitize_attributes", "auto_link", "auto_link_urls",
          "auto_link_email_addresses", "sanitize", "sanitize_with_escaped_attribute_brackets", "scrubbed", "scrub_attributes",
          "force_correct_attribute_escaping", "attrs", "attr", "qualified_name", "parse_nodes", "serialize_node",
          "defaults", "action_text", "content_filter", "sanitize_tags_allowed_tags", "sanitize_tags"]
rows = {}
for b in ["base", "view-3"]:
    out = subprocess.run(["perf", "report", "-i", f"{S}/perf-{b}.data", "--children", "--sort", "symbol", "--stdio", "-g", "none"],
                         capture_output=True, text=True).stdout
    total = int(re.search(r"Event count \(approx.\): (\d+)", out).group(1))
    agg = {}
    for line in out.splitlines():
        m = re.match(r"\s+([\d.]+)%\s+[\d.]+%\s+\[\.\] (.*?)\s{2,}", line)
        if not m:
            continue
        name = m.group(2)
        # "<campfire_richtext::dom::Dom>::attr", "attr (inlined)", "campfire_richtext::sanitizer::sanitize", "auto_link<..>"
        base = re.sub(r" \(inlined\)$", "", name)
        base = re.sub(r"<[^<>]*(<[^<>]*(<[^<>]*>[^<>]*)*>[^<>]*)*>$", "", base)  # trailing generics
        short = base.split("::")[-1]
        pct = float(m.group(1))
        if short in stages:
            agg[short] = max(agg.get(short, 0), pct)  # children: the outermost frame of that name
    rows[b] = (total, agg)
print("| stage | base Gcycles | VIEW-3 Gcycles | change |")
print("|---|---|---|---|")
tb, ab = rows["base"]; tv, av = rows["view-3"]
print(f"| whole run | {tb/1e9:.2f} | {tv/1e9:.2f} | {100*(tv-tb)/tb:+.1f}% |")
for s in stages:
    b = ab.get(s, 0) * tb / 100 / 1e9; v = av.get(s, 0) * tv / 100 / 1e9
    ch = f"{100*(v-b)/b:+.1f}%" if b else "n/a"
    print(f"| `{s}` | {b:.2f} | {v:.2f} | {ch} |")
