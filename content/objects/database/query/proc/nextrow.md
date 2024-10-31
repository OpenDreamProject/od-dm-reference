+++
title = "NextRow"
slug = "NextRow" # AUTOGEN FIELD
[extra.return]
type = "number"
description = "1 if the next row was read, 0 otherwise"
+++

This will shift the reading to the next row of results. It must also be called before reading the *first* result. After this has been called, you can use {{ database_query(proc="GetRowData") }} or {{ database_query(proc="GetColumn") }} to retrieve the data from this row.
