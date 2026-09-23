+++
title = "_dm_db_next_row"
[[extra.args]]
name = "db_query"
description = "A database query"
[[extra.args]]
name = "items"
description = "Container for the row items"
[[extra.args]]
name = "conversions"
description = "A list"
[extra.return]
type = "num" # AUTOGEN SKIP
description = "`TRUE` if another row was selected, otherwise `FALSE`"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++

{% parity() %}
Behaviour of `conversions` is currently unknown.
{% end %}

If possible, inserts the values of the next row into `items`.

Calling this proc repeatedly will continue stepping through rows until there are no more rows in the selection.<br>
It is not possible to step backwards.
