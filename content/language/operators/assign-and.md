+++
title = "&="

[extra]
usage = "x &= y"
+++

Assigns x to the value of `x & b`. It is equivalent to `x = x & b`. This is often used for disabling bitflags in a bitfield.

```dm
#define MY_BITFLAG (1<<0)

var/my_field = MY_BITFLAG // starts enabled
my_field &= ~MY_BITFLAG // now disabled
```

If both x and y are {{ list() }}s, items in x that are not present in y will be removed.

```dm
var/list/a = list("foo")
var/list/b = list("bar")

a &= b

world.log << json_encode(a) // empty list!
```
