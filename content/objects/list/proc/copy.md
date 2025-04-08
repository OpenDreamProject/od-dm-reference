+++
title = "Copy"
slug = "Copy" # AUTOGEN FIELD
[[extra.args]]
name = "Start" # AUTOGEN STATIC
default_value = "1" # AUTOGEN FIELD
description = "Index of the first element to be included in the copy"
[[extra.args]]
name = "End" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "Index past the last element to be included in the copy, or 0 for the last element"
[extra.return]
description = "A shallow copy of the list"
+++

Returns a new shallow copy of the list.

```dm
var/list/L1 = list(1, 2, 3)
var/list/L2 = L1.Copy(2)

world.log << json_encode(L2) // "[2,3]"
```