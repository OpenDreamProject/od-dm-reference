+++
title = "fract"
[[extra.args]]
name = "n" # AUTOGEN STATIC
description = "A number."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "The numbers after the decimal point."
+++

```dm
world.log << fract(0.1234) // 1234
world.log << fract(-0.1234) // -1234
```
