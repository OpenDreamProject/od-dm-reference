+++
title = "^="

[extra]
usage = "x ^= y"
+++

Assigns `x` to the value of `x ^ y`. It is equivalent to `x = x ^ y`.

If `x` and `y` are both {{ list() }}s, items found in both lists will be removed from `x`, and then only items present in `y` are added to `x`.
