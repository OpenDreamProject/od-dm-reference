+++
title = "|"

[extra]
usage = "x | y"
return = "The binary OR of x and y"
+++

If `x` and `y` are both {{ list() }}s, the result is a list containing items that are in either list. Items from list `x` will be first, followed by items from list `y`, if not present in `x`.
