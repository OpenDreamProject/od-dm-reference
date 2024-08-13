{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/atom/movable", destination="@/objects/atom/movable/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}