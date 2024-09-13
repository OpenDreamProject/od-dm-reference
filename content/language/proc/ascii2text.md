+++
title = "ascii2text"
[extra.return]
type = "text" # AUTOGEN FIELD
description = "The converted character"
[[extra.args]]
name = "N" # AUTOGEN STATIC
description = "number, character code"
+++

Returns the character corresponding to a UTF-32 character code - despite the name of this proc, it is not limited to ASCII characters.

```dm
world.log << ascii2text(5660) // ᘜ
```
