{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/obj", destination="@/objects/obj/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}