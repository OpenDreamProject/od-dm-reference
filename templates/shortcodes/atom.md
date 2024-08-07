{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/atom", destination="@/objects/atom/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}