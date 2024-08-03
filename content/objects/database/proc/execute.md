+++
title = "Execute"

[extra]
args = [
    { name = "db", type = "/database", description = "The database we are executing this query against." },
    ]

+++

Runs the query that has been built. This command must be executed prior to accessing any rows.

```dm
var/database/db = new("database.db")
var/database/query/query = new("CREATE TABLE players (id int, ckey string, points int)")

query.Execute(db)
```