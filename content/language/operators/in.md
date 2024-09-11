+++
title = "in"

[extra]
usage = "x in list"
return = "1 if x is in list, 0 otherwise"
+++

This is a null-safe way of checking if an item is in a list, unlike {{ list(proc="Find") }}, which will runtime error if the list is not defined.

To check if an item is **not** in a given list, you can check `!(item in list)`. This is distinct from checking `(!item in list)`, which would coerce item into 0 or 1, and check if 0 or 1 is in the list.
