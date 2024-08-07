{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/mob", destination="@/objects/mob/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}