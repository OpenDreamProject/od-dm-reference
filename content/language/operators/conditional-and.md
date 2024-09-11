+++
title = "&&"

[extra]
usage = "x && y"
return = "True value if x and y are both truthy, or the first falsy value encountered."
+++

The return value is the last argument to be evaluated. The first falsy value from left to right will short-circuit the expression, and the rest of the expression will not be evaluated.

```dm
var/foo = FALSE && blow_up_the_world()
```

This code will never end up blowing up the world, as when evaluating the expression, it will stop after the first falsy value (in this case, `FALSE`).
