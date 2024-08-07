{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/datum", destination="@/objects/datum/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}