{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/exception", destination="@/objects/exception/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}