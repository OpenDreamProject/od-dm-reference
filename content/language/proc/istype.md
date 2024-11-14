+++
title = "istype"
[extra]
format = [
    [{name = "Val"}],
    [{name = "Val", description = "An instantiated object"}, {name = "Type", description = "A type or object. If this argument is not specified, it will use the inferred type of the first argument."}]
]
[extra.return]
type = "num"
description = "1 if Val is the same as, or a child type of, Type"
+++

```dm
var/atom/my_atom = new /obj(loc) // the variable is declared as an /atom, but we create an /obj
if(istype(my_atom))
    // this will not pass, as the inferred type is different from the actual type

if(istype(my_atom, /obj))
    // this will pass, as we are providing a second argument to test with, which matches the type
```

For testing paths, use [ispath()](@/language/proc/ispath.md).
