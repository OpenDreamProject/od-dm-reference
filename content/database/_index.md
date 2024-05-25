+++
title = "/database"
+++

The `/database` datum allows for interfacing with an SQLite database, running SQL queries with `/database/query`. We currently support writing and reading DreamMaker floats and strings.

```js
var/database/database = new("test-database.db")

var/database/query/query = new()
query.Add("SELECT * FROM players WHERE ckey = ?", "my-player")

query.Execute(database)
query.NextRow()
query.GetRowData()
```