{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/dm_filter", destination="@/objects/dm_filter/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}