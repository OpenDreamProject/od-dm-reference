+++
title = "_dm_db_new_query"
[extra.return]
type = "db_query" # AUTOGEN SKIP
description = "A new query object"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++
Creates and returns an internal object we call a `db_query`.

`db_query` is a special data type, different from a {{ datum() }} but still treated as an object<br>
It is used to store the results of a [query](./_dm_db_execute.md).