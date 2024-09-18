+++
title = "New"
slug = "New" # AUTOGEN FIELD
[extra]
is_override = true # AUTOGEN FIELD
format = [[{name = "text", description = "The command text to add to the query."}, {name = "item1", description = "Parameters to substitute into the command text."}, {name = "item2"}, {name = "..."}]]
[[extra.args]]
name = "text" # AUTOGEN STATIC
+++

This will create a new query. If the arguments to the proc are provided, {{ database_query(proc="Add") }} will automatically be called.
