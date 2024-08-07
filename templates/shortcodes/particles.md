{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/particles", destination="@/objects/particles/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}