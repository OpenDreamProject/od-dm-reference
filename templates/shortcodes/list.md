{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/list", destination="@/objects/list/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}