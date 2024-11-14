+++
title = "isfile"
[[extra.args]]
name = "File" # AUTOGEN STATIC
description = "The potential file to check."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 for a file; 0 if it is not."
+++

This works for both resource files, and files retrieved via [file()](@/language/proc/file.md).

```dm
var/my_resource = 'icons/turf.dmi'

if(isfile(my_resource))
    world.log << "It's true!"
```
