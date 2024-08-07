{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/world", destination="@/objects/world/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}