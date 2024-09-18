+++
title = "Add"
slug = "Add" # AUTOGEN FIELD
[extra]
format = [[{name = "text", description = "The command text to use for the query."}, {name = "item1", description = "Parameters to substitute into the command text."}, {name = "item2"}, {name = "..."}]]
[[extra.args]]
name = "text" # AUTOGEN STATIC
+++

This adds the command text - the actual SQL query - to a database datum. Existing command text will be removed automatically - this is equivalent to running {{ database_query(proc="Clear") }}.

```dm
var/database/my_db = new("sqlite.db")
var/database/query/my_query = new()
my_query.Add("CREATE TABLE my_table (id int, name string)")
my_query.Execute(my_db)
```

When additional arguments are given to this proc, they will be used to substitute `?` in the command text. This is functionally equivalent to prepared statements.

```dm
var/database/query/my_other_query = new()
my_other_query.Add("INSERT INTO my_table (name) VALUES (?)", "Barry")
```

This is equivalent to the following:

```sql
INSERT INTO my_table (name) VALUES ("Barry")
```
