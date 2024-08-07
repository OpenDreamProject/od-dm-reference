{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/turf", destination="@/objects/turf/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}