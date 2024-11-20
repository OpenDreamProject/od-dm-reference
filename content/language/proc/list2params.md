+++
title = "list2params"
[[extra.args]]
name = "List" # AUTOGEN STATIC
description = "The list to encoded as a parameter string."
[extra.return]
type = "text" # AUTOGEN FIELD
description = "A parameter string."
+++

This converts an [associated list](@/objects/list/association.md) to a parameter string, for use in {{ client(proc="Topic") }}, among other places.

```dm
var/my_list = list("some_key" = 121, "some_other_key" = "some other value")

world.log << list2params(my_list) // some_key=121&some_other_key=some+other+value
```

Characters that have a special meaning in a parameter string, such as `=` and `&`, will be [percent-encoded](https://developer.mozilla.org/en-US/docs/Glossary/Percent-encoding).

{% parity() %}
In BYOND, references to datums are automatically converted into a text reference, equivalent to running `\ref[datum]`.
{% end %}
