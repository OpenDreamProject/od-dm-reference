{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/generator", destination="@/objects/generator/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}