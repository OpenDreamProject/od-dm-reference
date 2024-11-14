+++
title = "ftime"
[[extra.args]]
name = "File" # AUTOGEN STATIC
description = "The path to the entry on the file system to test."
[[extra.args]]
name = "IsCreationTime" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "1 return the creation date; 0 for last modified date"
[extra.return]
type = "num" # AUTOGEN FIELD
description = "Date expressed as number of deciseconds since Jan 1, 2000."
+++

This value can be formatted using [time2text()](@/language/proc/time2text.md).

```dm
var/time = ftime("environment.dme") // 7.76174e+09
world.log << time2text(time) // Mon Aug 05 12:58:22 2024
```
