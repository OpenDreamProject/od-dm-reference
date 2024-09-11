+++
title = "?."

[extra]
usage = "x?.property"
+++

This is used to access the properties of an object, like the [. operator](@/language/operators/dot.md). However, using this operator, if the object is null, the access does not occur, and no runtime error will happen.

```dm
var/obj/my_thingy // not assigned to anything, so this will be null
my_thingy?.name = "Balloon" // as it is null, no access occurs, and no runtime occurs

var/obj/my_other_thingy = new /obj()
my_other_thingy?.name = "Runtime Damian" // as it is not null, and /obj has a member called name, the variable will be updated
```

This behavior is identical to the [?: operator](@/language/operators/nullsafe-colon.md), but that operator will not check at compile time if the property does exist for the provided variable type. As such, the `?.` operator is generally preferred.
