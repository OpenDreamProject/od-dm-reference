+++
title = "?:"

[extra]
usage = "x?:property"
+++

This is used to access the properties of an object without compile time safety-checks, like the [: operator](@/language/operators/colon.md). However, using this operator, if the object is null, the access does not occur, and no runtime error will happen.

```dm
var/my_thingy // not assigned to anything, so this will be null
my_thingy?:name = "Damian" // as it is null, no access occurs, and no runtime occurs

var/my_other_thingy = 1 // assigned to a truthy value
my_other_thingy?:name = "Runtime Damian" // as it is not null, access does occur, and 1 does not have a name variable, so a runtime occurs
```

This behavior is identical to the [?. operator](@/language/operators/nullsafe-dot.md), but that operator will check if the property is valid for the variable's type. As such, that operator is typically preferred, as it will provide compile-time errors.
