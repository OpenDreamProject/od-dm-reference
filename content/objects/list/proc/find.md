+++
title = "Find"
slug = "Find" # AUTOGEN FIELD
[[extra.args]]
name = "Elem" # AUTOGEN STATIC
[[extra.args]]
name = "Start" # AUTOGEN STATIC
default_value = "1" # AUTOGEN FIELD
description = "Index of the first element to be included in the search"
[[extra.args]]
name = "End" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "Index past the last element to be included in the search, or 0 for the last element"
[extra.return]
type = "num"
description = "The index of the found element, or 0"
+++

Search for the given item in the list. If any values in the list are equal, this will return its index. Otherwise it will return 0.

```dm
var/list/L = list(10, 20, 30, 20)

// Will return the first instance of 20
world.log << L.Find(20) // 2
```