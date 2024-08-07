{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/database", destination="@/objects/database/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}