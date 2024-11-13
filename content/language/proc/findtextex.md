+++
title = "findtextEx"
slug = "findtextEx" # AUTOGEN FIELD
[[extra.args]]
name = "Haystack" # AUTOGEN STATIC
description = "The string to search through."
[[extra.args]]
name = "Needle" # AUTOGEN STATIC
description = "The text to search for, or the /regex to search with."
[[extra.args]]
name = "Start" # AUTOGEN STATIC
default_value = "1" # AUTOGEN FIELD
description = "The byte position in the string to start searching at."
[[extra.args]]
name = "End" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "The byte position immediately after the last character that should be searched."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "The position in the string of the first match; 0 if no match was found."
+++

If the Needle provided is a string, the comparison occurs case sensitively. For the case insensitive version, see [findtext.md](@/language/proc/findtext.md).

```dm
if(findtext("Foo Bar", "foo"))
    world.log << "This fails!"
```