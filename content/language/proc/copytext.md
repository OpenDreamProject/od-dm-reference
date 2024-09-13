+++
title = "copytext"
[extra.return]
type = "null, text" # AUTOGEN FIELD
description = "A new string copied from the provided points"
[[extra.args]]
name = "T" # AUTOGEN STATIC
description = "The text string to copy from"
[[extra.args]]
name = "Start" # AUTOGEN STATIC
default_value = "1" # AUTOGEN FIELD
description = "The position in the string to start the copy"
[[extra.args]]
name = "End" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "The position in the string to finish the copy, exclusive"
+++

Copies characters in the provided string between the Start and End position. By default, the End value will copy the remainder of the string. You can also specify negative positions for the Start or End, which will cause it to work backwards.

```dm
world.log << copytext("Foo Bar", 5) // Bar
world.log << copytext("Foo Bar", 1, 4) // Foo

world.log << copytext("Foo Bar", -3) // Bar
world.log << copytext("Foo Bar", -7, -3) // Foo
```

For non-ASCII strings, where the string position does not necessarily correlate to character position, you will need the `_char` variant of this proc. `copytext_char()` works in text elements, and otherwise functions identically.
