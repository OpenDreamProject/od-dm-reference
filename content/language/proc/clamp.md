+++
title = "clamp"
[extra.return]
type = "null, num, /list" # AUTOGEN FIELD
description = '''The provided number, kept between the Low and High values.
The original list, with the numeric contents kept between the Low and High values.
'''
[extra]
format = [
  [
    { name = "Number", description = "A number." },
    { name = "Low", description = "The lowest number that can be returned." },
    { name = "High", description = "The highest number that can be returned." },
  ],
  [
    { name = "List", type = "/list", description = "A list of numbers." },
    { name = "Low" },
    { name = "High" },
  ],
]
[[extra.args]]
name = "Value" # AUTOGEN STATIC
[[extra.args]]
name = "Low" # AUTOGEN STATIC
[[extra.args]]
name = "High" # AUTOGEN STATIC
+++

Constricts a number between the Low and High values provided. If the number is between those values, it is unchanged. If it exceeds the Low or High values, those will be returned.

```dm
world.log << clamp(5, 1, 10) // 5
world.log << clamp(1, 5, 10) // 5
world.log << clamp(15, 1, 10) // 10
```

When operating on a list, this functions as if `clamp()` has been run on each entry in the list.

```dm
var/my_list = list(1, 2, 3, 4, 5, 6, 7, 8, 9)
for(var/num in clamp(my_list, 1, 5))
    world.log << num // 1, 2, 3, 4, 5, 5, 5, 5, 5
```

{% parity() %}
A new list will always be returned from the `clamp()` proc, it does not operate in place
{% end %}
