+++
title = "GetColumn"
slug = "GetColumn" # AUTOGEN FIELD
[extra.return]
description = "The contents of the given column"
[[extra.args]]
name = "column" # AUTOGEN STATIC
description = "The 0-based column number to retrieve information from"
+++

Once the query has been run with {{ database_query(proc="Execute") }} and we have started to read rows with {{ database_query(proc="NextRow") }}, we can retrieve the data stored in a specific column with this proc. To get the names of the columns, {{ database_query(proc="Columns") }} can be used.
