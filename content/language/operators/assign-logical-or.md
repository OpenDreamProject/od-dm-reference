+++
title = "|="

[extra]
usage = "x |= y"
+++

Assigns x to the value of `x | y`. It is equivalent to `x = x | y`. This is often used for enabling bitflags in a bitfield.

```dm
#define MY_BITFLAG (1<<0)

var/my_field = 0 // starts disabled
my_field |= MY_BITFLAG // now enabled
```

If both x and y are {{ list() }}s, all items in y not already present in x will be added to x.
