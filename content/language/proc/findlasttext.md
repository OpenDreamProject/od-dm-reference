+++
title = "findlasttext"
[[extra.args]]
name = "Haystack" # AUTOGEN STATIC
description = "The string to search through."
[[extra.args]]
name = "Needle" # AUTOGEN STATIC
description = "The text to search for."
[[extra.args]]
name = "Start" # AUTOGEN STATIC
default_value = "1" # AUTOGEN FIELD
description = "The byte position in the string to start searching at. As this is searching backwards, this defaults to the end of the string."
[[extra.args]]
name = "End" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "The byte position immediately before the last character that should be searched."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "The position in the string of the first match; 0 if no match was found."
+++

The comparison occurs case insensitively - for the case sensitive version, see [findlasttextex.md](@/language/proc/findlasttextex.md).

```dm
world.log << findlasttext("FooFoo", "foo") // 4
world.log << findlasttext("FooFoo", "foo", 3) // 1
```