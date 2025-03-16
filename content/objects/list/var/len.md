+++
title = "len"
[extra]
default_value = "" # AUTOGEN FIELD
is_override = false # AUTOGEN FIELD
+++

The length of the list. Accessing this gives you the amount of values the list contains.

Changing this value resizes the list, which will remove elements beyond the specified length if the list is shrunk, or grow the list with null values.

```dm
var/list/my_list = list("foo", "bar", "car")
my_list.len = 2

for(var/element in my_list)
    world.log << element // "foo", "bar"

var/list/my_other_list = list("foo")
my_other_list.len = 3

for(var/element in my_list)
    world.log << isnull(element) // FALSE (0), TRUE (1), TRUE (1)
```