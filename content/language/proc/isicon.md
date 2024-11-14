+++
title = "isicon"
[[extra.args]]
name = "Icon" # AUTOGEN STATIC
description = "The potential icon to check."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 for an icon; 0 if it is not."
+++

This works for both {{ icon() }} objects, and icon files loaded into the resource cache.

```dm
if(isicon('sounds/zworp.ogg'))
    // this will not pass

if(isicon('icons/alien.dmi'))
    // but this will
```
