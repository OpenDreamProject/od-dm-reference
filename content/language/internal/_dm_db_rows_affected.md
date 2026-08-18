+++
title = "_dm_db_rows_affected"
[[extra.args]]
name = "db_query"
description = "A database query"
[extra.return]
type = "num, null" # AUTOGEN SKIP
description = "The number of rows, or `null` if not applicable"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++

Returns the number of rows that were **altered** by the query's execution.