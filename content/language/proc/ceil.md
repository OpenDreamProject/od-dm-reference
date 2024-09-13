+++
title = "ceil"
[extra]
is_override = false # AUTOGEN FIELD
[extra.return]
type = "num" # AUTOGEN FIELD
description = "The ceiling of the provided number."
[[extra.args]]
name = "A" # AUTOGEN STATIC
description = "Number"
+++

The ceiling is the largest integer greater than or equal to the provided number.

```dm
world.log << ceil(1.00001) // 2
world.log << ceil(-0.00001) // 0
```
