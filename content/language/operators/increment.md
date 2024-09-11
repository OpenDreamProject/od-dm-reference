+++
title = "++"

[extra]
usage = "++x (pre-increment), x++ (post-increment)"
+++

The pre-increment has the value of `x + 1`, and adds 1 to x.

The post-increment has the value of `x`, and then adds 1 to x.

```dm
var/x = 0
world.log << ++x // 1
world.log << x++ // 1
world.log << x // 2
```
