{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/mutable_appearance", destination="@/objects/mutable_appearance/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}