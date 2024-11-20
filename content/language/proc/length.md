+++
title = "length"
[extra.return]
type = "number, null"
description = "The length of the provided object."
[[extra.args]]
name = "object"
description = "A string, a list or a file."
+++

```dm
world.log << length("Hello, world!") // 13, number of characters

world.log << length(list("Hello,", " world!")) // 2, number of elements in the array
```

{% parity() %}
The BYOND implementation of DreamMaker also allows you to check the length of a file, such as:

```dm
world.log << length(file("foo.txt"))
```

{% end %}

For non-ASCII strings, you can use the `length_char()` variant of this proc to correctly handle unicode, which works with character counts.
