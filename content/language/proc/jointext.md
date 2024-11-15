+++
title = "jointext"
[[extra.args]]
name = "List" # AUTOGEN STATIC
type = "/list" # AUTOGEN FIELD
description = "The list of elements to join up."
[[extra.args]]
name = "Glue" # AUTOGEN STATIC
description = "The text to be inserted between elements."
[[extra.args]]
name = "Start" # AUTOGEN STATIC
default_value = "1" # AUTOGEN FIELD
description = "The index in the list to start at."
[[extra.args]]
name = "End" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "The index after the final element to join."
[extra.return]
type = "text" # AUTOGEN FIELD
description = "A string of the elements in List, joined together by the Glue."
+++

All items in the list are stringified - similar to running `"[element]"` over each element.

```dm
    world.log << jointext(list(1, "waaa", 'icons.dmi'), ",") // "1,waaa,icons.dmi"
```

Negative amounts for the Start or End value count from the end of the list.

This functions similarly to {{ list(proc="Join") }}.
