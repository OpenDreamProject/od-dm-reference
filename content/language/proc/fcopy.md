+++
title = "fcopy"
[[extra.args]]
name = "Src" # AUTOGEN STATIC
description = "The file to copy."
[[extra.args]]
name = "Dst" # AUTOGEN STATIC
description = "Where the file should be copied to."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 on success; 0 on failure."
+++

The source file can be a savefile, a file on the filesystem or a loaded resource, such as an icon. 

```dm
// resources are specified with 'single quotes'
fcopy('icons/mob.dmi', "temp/mob.dmi")

// "double quotes" specify an external file
fcopy("temp/mob.dmi", "the_void/")
```

{% parity() %}
In BYOND, if both the source and destination have a trailing slash, they will be treated as directories. This recursively copies the contents of directories into the target path.
{% end %}