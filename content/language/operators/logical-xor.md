+++
title = "^"

[extra]
usage = "x | y"
return = "The binary XOR of x and y"
+++

If `x` and `y` are both {{ list() }}s, the result is a list containing items that are in either list, but not in both lists. Items from only `x` will be first, followed by items from only list `y`.
