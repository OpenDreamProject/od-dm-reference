{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/sound", destination="@/objects/sound/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}