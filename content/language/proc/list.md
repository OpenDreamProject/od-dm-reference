+++
title = "list"
[extra]
format = [
  [
    { name = "value1" },
    { name = "value2" },
    { name = "value3" },
    { name = "..." },
  ],
  [
    { name = "key1 = value1" },
    { name = "key2 = value2" },
    { name = "key3 = value3" },
    { name = "..." },
  ],
]
return = { description = "The values provided, inserted into a new {{ list() }}." }
args = [{ name = "..." }]
+++

Creates a new {{ list() }} with the values provided as arguments. If the values provided have both keys and values, an [associated list](@/objects/list/association.md) will be created.

```dm
for(var/element in list(1, 2, 3))
    world.log << element // 1, 2, 3

var/assoc_list = list("foo" = 1, "bar" = 2, "car" = 3)
for(var/key in assoc_list)
    world.log << key // foo, bar, car
    world.log << assoc_list[key] // 1, 2, 3
```
