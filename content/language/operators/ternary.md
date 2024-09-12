+++
title = "?"

[extra]
usage = "x ? do_when_true() : do_when_false()"
+++

If `x` evaluates to a truthy value, `do_when_true()` will be evaluated and returned. Otherwise, `do_when_false()` will be evaluated and returned.

```dm
var/my_number = pick(1, 2)
world.log << my_number == 2 ? "My number was two!" : "My number was not two."
```
