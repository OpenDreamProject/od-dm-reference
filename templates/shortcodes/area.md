{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/area", destination="@/objects/area/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}