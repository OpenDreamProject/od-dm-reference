{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/icon", destination="@/objects/icon/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}