+++
title = "get_steps_to"
[extra]
od_unimplemented = true # AUTOGEN FIELD
[[extra.args]]
name = "Ref" # AUTOGEN STATIC
description = "The moving object."
[[extra.args]]
name = "Trg" # AUTOGEN STATIC
description = "The object being moved towards."
[[extra.args]]
name = "Min" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "If the two objects are within this many steps, no step is calculated."
[extra.return]
type = "/list, null" # AUTOGEN SKIP
description = "A list of directions to step in to reach the Trg; or null if no path is available."
+++

Calculates a path from Ref to Trg, avoiding dense objects. If the target is more than double {{ world(var="view") }}, null is returned.

```dm
for(var/dir in get_steps_to(me, them))
    step(me, dir) // on our way!
```
