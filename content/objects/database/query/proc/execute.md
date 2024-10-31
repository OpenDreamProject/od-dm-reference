+++
title = "Execute"
slug = "Execute" # AUTOGEN FIELD
[[extra.args]]
name = "database" # AUTOGEN STATIC
+++

Runs the generated query against the provided database. If the query is expected to return data, you can increment the row being read with {{ database_query(proc="NextRow") }}.

```dm
var/database/db = new("mydb.db")
var/database/query/query = new("SELECT user FROM leaderboard ORDER BY score ASC")
query.Execute(db)

var/list/top_scorers = list()

while(query.NextRow())
    top_scorers += query.GetColumn(0)
```

{{ parity(description="We do not currently support running Execute() without a database datum provided. The BYOND implementation of DreamMaker supports passing a filename.") }}
