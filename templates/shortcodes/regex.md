{% import "shortcodes.html" as sc %}
{{ sc::render_link(type="/regex", destination="@/objects/regex/_index.md", proc=proc | default(value=false), var=var | default(value=false)) }}