+++
title = "ascii2text"
[extra]
return_type = "text" # AUTOGEN FIELD
return_type_desc = "The converted character"
is_override = false # AUTOGEN FIELD
[[extra.args]]
name = "N" # AUTOGEN STATIC
description = "number, character code"
+++

Returns the character corresponding to a UTF-32 character code - despite the name of this proc, it is not limited to ASCII characters.

```dm
world.log << ascii2text(5660) // ᘜ
```
