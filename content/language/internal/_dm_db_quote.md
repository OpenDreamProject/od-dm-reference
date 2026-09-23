+++
title = "_dm_db_quote"
[[extra.args]]
name = "db_conn"
description = "A database connection"
[extra.return]
type = "text" # AUTOGEN SKIP
description = "Sanitized form of the input text"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++

Asks the connection to escape the provided text (Ex: `this 'dangerous' text` to `this \\'dangerous\\' text`).<br>
Very important for treating user input, as it can prevent [malicious attacks](https://en.wikipedia.org/wiki/SQL_injection) on databases.