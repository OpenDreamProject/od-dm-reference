+++
title = "--"

[extra]
usage = "--x (pre-decrement), x-- (post-decrement)"
+++

The pre-decrement has the value of `x - 1`, and subtracts 1 from x.

The post-decrement has the value of `x`, and then subtracts 1 from x.

```dm
var/x = 1
world.log << --x // 0
world.log << x-- // 0
world.log << x // -1
```
