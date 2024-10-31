+++
title = "GetRowData"
slug = "GetRowData" # AUTOGEN FIELD
[extra.return]
type = "/list"
description = "The data stored in the current row, with the key being the column name and the value being the row value"
+++

This retrieves the information of the current row presented in an [associated list](@/objects/list/association.md). This can only be run after the query has been executed with {{ database_query(proc="Execute") }} and started being read with {{ database_query(proc="NextRow") }}.

```dm
var/database/db = new("database.db")
var/database/query/query = new("SELECT * FROM players")
query.Execute()
query.NextRow()

var/row = query.GetRowData()
for(var/key in row)
    world.log << "[key]: [row[key]]" // 'username: "Barry"', 'score: 500', 'admin: 0'
```
