+++
title = "_dm_db_close"
[[extra.args]]
name = "db_obj"
description = "An internal database object, db_conn or db_query"
[extra.return]
type = "num" # AUTOGEN SKIP
description = "`TRUE` if the object was closed, otherwise `FALSE`"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++

"Closes" the [`db_conn`](./_dm_db_new_con.md) or [`db_query`](./_dm_db_new_query.md), resetting the object to its initial state.

It's important to ensure that `db_conn` instances always closes its connection at the end of its lifetime,<br>
as failing to do so can cause hanging connections that are impossible to close.