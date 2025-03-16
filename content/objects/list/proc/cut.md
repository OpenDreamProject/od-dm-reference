+++
title = "Cut"
slug = "Cut" # AUTOGEN FIELD
[[extra.args]]
name = "Start" # AUTOGEN STATIC
default_value = "1" # AUTOGEN FIELD
description = "Index of the first element to be cut from the list"
[[extra.args]]
name = "End" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "Index past the last element to be cut from the list, or 0 for the last element"
+++

Removes the indicated elements from the list.

```dm
var/list/L = list(1, 2, 3, 4)
L.Cut(2, 4)

world.log << json_encode(L) // "[1,4]"
```