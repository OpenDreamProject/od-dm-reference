+++
title = ":"
+++

This is used to access the properties of a variable that it is not necessarily cast as having. If the variable does not contain the specified property, a runtime error will occur.

```dm
var/foo = null

foo:name = "yay!"
```

This code compiles, however, upon execution of the code, it will error, as you cannot access the `name` var of null.

The [. (dot)](@/language/operators/dot.md) operator works the same way to access a variable's properties, however, it will check at compile time if the type the variable is cast as contains this property. This is preferred behavior - it is easier to catch errors in the compiler than at runtime.
