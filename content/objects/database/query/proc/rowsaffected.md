+++
title = "RowsAffected"
slug = "RowsAffected" # AUTOGEN FIELD
[extra.return]
type = "number"
description = "The number of rows impacted by the previous operation"
+++

After running a query that makes changes to the database, this can be used to determine if the operation has succeeded.

```dm
var/database/db = new("db.db")
var/database/query/query = new("UPDATE players SET upgraded = 1 WHERE score > 500")
query.Execute(db)

var/affected = query.RowsAffected()
if(affected)
    world.log << "[affected] players have been upgraded."
```
