+++
title = "cmptext"
[extra]
format = [[{ name = "string1" }, { name = "string2" }, { name = "..." }]]
is_override = false # AUTOGEN FIELD
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 if all the provided arguments are the same, 0 otherwise"
[[extra.args]]
name = "T1" # AUTOGEN STATIC
description = "A text string to compare against."
+++

This proc is case insensitive, unlike [cmpTextEx()](@/language/proc/cmptextex.md). Any number of arguments can be given to this proc, and it will return 1 if they are all the same.

```dm
if(cmptext("letsplayagame", "LETSPLAYAGAME", "letsPlayAGame"))
    world.log << "Yay!" // "Yay!" is printed
```
