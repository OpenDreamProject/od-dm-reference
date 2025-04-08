+++
title = "Add"
slug = "Add" # AUTOGEN FIELD
[[extra.args]]
name = "Item1, Item2, ..."
description = "One or more values to add"
+++

Adds a value, or multiple values, to the list.

```dm
var/list/L = list(1, 2)

L.Add(3)
world.log << json_encode(L) // "[1,2,3]"

L.Add(4, 5)
world.log << json_encode(L) // "[1,2,3,4,5]"
```

If any of the values are another list then the contents of that list will be added rather than the list itself.

```dm
var/list/L1 = list(1, 2)
var/list/L2 = list(3, 4)

L1.Add(L2)

// "[1,2,3,4]"
// NOT "[1,2,[3,4]]"
world.log << json_encode(L1)
```