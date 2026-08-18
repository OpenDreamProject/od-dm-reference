+++
title = "_dm_db_columns"
[[extra.args]]
name = "db_query"
description = "A database query"
[[extra.args]]
name = "column_type"
description = "Path that represents column data"
[extra.return]
type = "list" # AUTOGEN SKIP
description = "An associative list of text keys to instance values"

[extra]
od_unimplemented = true # AUTOGEN FIELD
+++

Once a query is [executed](./_dm_db_execute.md), the `db_query` can be used to create a list of names along with their column data, which is useful for iterating over a set of rows.

Each entry will have the name of the column as the key and an instance of the provided path as the value.
For example, the query:
```sql
SELECT `first_name`, `last_name`, `dob` IN `my_table`;
```
will create this list:
```dm
list(
	"first_name" = new /column(...),
	"last_name" = new /column(...),
	"dob" = new /column(...),
)
```

Each database instance is created with **index-based** arguments.
The arguments that are provided will differ based on the database driver.

## MySql
```dm
/column/New(
	name, // column name
	table, // also column name
	position, // column index, 0-index (BYOND is 1-index)
	type, // appears to always be 0
	flag, // column definition flags
	length, // appears to always be a huge number
	max_length // appears to always be a huge number
)
```