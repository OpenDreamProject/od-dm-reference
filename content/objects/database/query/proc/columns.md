+++
title = "Columns"
slug = "Columns" # AUTOGEN FIELD
[extra.return]
description = """A /list of column names for the current query
The column name of the provided numbered column
"""
[[extra.args]]
name = "column" # AUTOGEN STATIC
description = "Optional, the numbered column to read from"
+++

Once a query has started being read (after {{ database_query(proc="Execute") }} has been called), the column names can be retrieved using this proc.

```dm
query.Add("CREATE TABLE example (id int, name string, foo string, bar string)")
query.Execute(db)

query.Add("SELECT * FROM example")
query.Execute(db)

var/columns = query.Columns() // "id", "name", "foo", "bar"
var/specific_column = query.Columns(0) // "id"
```
