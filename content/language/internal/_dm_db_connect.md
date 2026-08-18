+++
title = "_dm_db_connect"
[[extra.args]]
name = "db_conn"
description = "A database connection"
[[extra.args]]
name = "dsn"
description = "The data source name (DSN) of the connection"
[[extra.args]]
name = "username"
description = "Username for login credentials"
[[extra.args]]
name = "password"
description = "Password for login credentials"
[[extra.args]]
name = "cursor_type"
description = "Cursor type that the query will use"
[[extra.args]]
name = "arg_6"
od_unimplemented = true
[extra.return]
type = "num" # AUTOGEN SKIP
description = "`TRUE` if a connection was established successfully, otherwise `FALSE`"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++

Establishes a remote connection to a database.

The format for a DSN in BYOND is:<br>
```
dbi:[driver]:[identifier]:[address]:[port]
dbi:mysql:my_database:192.168.0.1:3306
```

Database drivers currently supported by BYOND:
- MySQL (`mysql`)

{% parity() %}
It is currently unknown if the database changes the values for `cursor_type`.
{% end %}

`cursor_type` changes how the query is handled:
- `0` uses the database's default cursor behaviour.
- `1` uses client cursor.
- `2` uses server cursor.