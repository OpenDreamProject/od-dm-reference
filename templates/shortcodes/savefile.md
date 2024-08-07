{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/savefile", destination="@/objects/savefile/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}