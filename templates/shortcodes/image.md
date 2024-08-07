{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/image", destination="@/objects/image/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}