{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/database-query", destination="@/objects/database/query/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}