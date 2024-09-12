+++
title = "?[]"

[extra]
usage = "x?[y], where x is a list and y is an index/key"
+++

This is the null-safe indexing operator, where if the list is null, the indexing operation will not occur.

```dm
var/list/my_list = list("foo", "bar")
var/list/my_nonexistent_list

world.log << my_list?[1] // "foo"
world.log << my_list?[1] // nothing!
```

If there is an expression on the right hand side of the operator to be evaluated and the list is null, the expression will not be evaluated. This will only take place if the list can be indexed.

```dm
my_nonexistent_list?[1] = do_something() // the proc will not be called here
my_list?[1] = do_something() // but it will be called here
```
