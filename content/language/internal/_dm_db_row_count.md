+++
title = "_dm_db_row_count"
[extra]
od_unimplemented = true # AUTOGEN FIELD
[[extra.args]]
name = "db_query"
description = "A database query"
[extra.return]
type = "num, null" # AUTOGEN SKIP
description = "The number of rows, or `null` if not applicable"
+++

Returns the number of rows that were **found** by the query's execution.