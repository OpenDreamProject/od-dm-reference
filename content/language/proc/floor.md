+++
title = "floor"
[[extra.args]]
name = "A" # AUTOGEN STATIC
description = "A number."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "The largest integer less than or equal to A."
+++

```dm
world.log << floor(0.999999) // 0
world.log << floor(-0.00001) // 1
```
