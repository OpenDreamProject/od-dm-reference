+++
title = "_dm_db_execute"
[[extra.args]]
name = "db_query"
description = "A database query"
[[extra.args]]
name = "query"
description = "The query being sent to the database"
[[extra.args]]
name = "db_conn"
description = "A database connection"
[[extra.args]]
name = "cursor_type"
description = "Cursor type that the query will use"
[[extra.args]]
name = "?"
description = "The purpose of this argument is unknown"
[extra.return]
type = "num" # AUTOGEN SKIP
description = "`TRUE` if the query executed successfully, otherwise `FALSE`"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++

Performs a query on a [`db_conn`](./_dm_db_new_con.md) and stores the result in a [`db_query`](./_dm_db_new_query.md).<br>

{% parity() %}
It is currently unknown if the database changes the values for `cursor_type`.
{% end %}

`cursor_type` changes how the query is handled:
- `0` uses the database's default cursor behaviour.
- `1` uses client cursor.
- `2` uses server cursor.