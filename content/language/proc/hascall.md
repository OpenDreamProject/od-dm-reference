+++
title = "hascall"
[[extra.args]]
name = "Object" # AUTOGEN STATIC
description = "The object to check."
[[extra.args]]
name = "ProcName" # AUTOGEN STATIC
type = "text"
description = "The name of the proc."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 if the object has the specified proc; 0 if it does not"
+++

```dm
/datum/my_thingy/proc/does_something()

/world/New()
    var/datum/my_thingy/thing = new

    if(hascall(thing, "does_something"))
        world.log << "The thing does something!"
```
