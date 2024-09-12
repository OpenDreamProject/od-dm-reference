+++
title = "[]"

[extra]
usage = "list[x]"
+++

This is used to access the element of lists with a given key or index.

```dm
var/list/my_list = list("foo")
world.log << my_list[1] // "foo"

var/list/my_other_list = list("key" = "bar")
world.log << my_other_list["key"] // "bar"
```
