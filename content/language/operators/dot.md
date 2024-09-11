+++
title = "."
+++

This is used to access the properties (procs and vars) of an object. This checks at compile time to see if the property being accessed does exist on the type of variable provided, providing build-time safety over the [: (colon)](@/language/operators/colon.md) operator.

```dm
var/mob/my_mob = new /mob()
my_mob.name = "Barry"
```

This successfully compiles *and* does not generate a runtime, as the type is correctly cast to {{ mob() }}, and the prototype in the variable is also a {{ mob() }}.

```dm
var/mob/my_mob
my_mob.name = "Evil Barry"
```

This code also successfully compiles, as the type is correctly cast to {{ mob() }}, which has an inherited variable called {{ atom(var="name") }}. However, there is no {{ mob() }} object assigned to the variable, so it will result in a runtime error.
