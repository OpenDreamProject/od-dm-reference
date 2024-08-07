+++
title = "/database"
template = "object.html"
weight = 4

[extra]
parent_type = "/datum"
+++

The `/database` datum allows for interfacing with an SQLite database, running SQL queries with {{ database_query() }}. We currently support writing and reading DreamMaker floats and strings.

```dm
var/database/database = new("test-database.db")

var/database/query/query = new()
query.Add("SELECT * FROM players WHERE ckey = ?", "my-player")

query.Execute(database)
query.NextRow()
query.GetRowData()
```