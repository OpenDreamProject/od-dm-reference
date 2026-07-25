+++
title = "_dm_db_new_con"
[extra.return]
type = "db_conn" # AUTOGEN SKIP
description = "A new connection object"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++
Creates and returns an internal object we call a `db_conn`.

`db_conn` is a special data type, different from a {{ datum() }} but still treated as an object.<br>
It is used to perform database operations.

{{ database() }} also internally creates a `db_conn`.