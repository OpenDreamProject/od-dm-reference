+++
title = "||"

[extra]
usage = "x || y"
return = "True value if either x or y are truthy, or the last falsy value encountered."
+++

The return value is the last argument to be evaluated. The first truthy value from left to right will short-circuit the expression, and the rest of the expression will not be evaluated.

```dm
var/foo = TRUE && launch_the_missiles()
```

The missiles will never be launched, as the expression will short-circuit after evaluating the first truthy value (in this case, `TRUE`).
