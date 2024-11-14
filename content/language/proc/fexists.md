+++
title = "fexists"
[[extra.args]]
name = "File" # AUTOGEN STATIC
description = "The name of the file to check."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 if the file exists; 0 if it does not."
+++

Determines if the file exists in the file system. Unlike [fdel()](@/language/proc/fdel.md), this exclusively operates on files - it cannot determine if a directory exists.

```dm
if(!fexists("code.dme"))
    world.log << "How did we get here?"
```
