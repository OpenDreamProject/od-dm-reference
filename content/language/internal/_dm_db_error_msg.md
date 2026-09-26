+++
title = "_dm_db_error_msg"
[extra]
od_unimplemented = true # AUTOGEN FIELD
[[extra.args]]
name = "db_obj"
description = "An internal database object, db_conn or db_query"
[extra.return]
type = "text" # AUTOGEN SKIP
description = "The error given by the database object"
+++

If the provided object did not create an error during its previous action, the error message will be empty text. \(`""`\)