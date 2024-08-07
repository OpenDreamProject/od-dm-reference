{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/client", destination="@/objects/client/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}