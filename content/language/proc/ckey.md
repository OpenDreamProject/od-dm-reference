+++
title = "ckey"
[extra.return]
type = "null, text" # AUTOGEN FIELD
description = "The canonical form of the provided string"
[[extra.args]]
name = "Key" # AUTOGEN STATIC
description = "The string to canonicalize"
+++

The canonical form of a given key is composed of lowercase letters between A-Z, numbers between 0-9 and the `@` symbol. This is pre-generated on both {{ mob(var="ckey") }} and {{ client(var="ckey") }}, based on the user's {{ client(var="key") }}.

```dm
var/my_evil_key = "W0w!!IGotLots-ofFunCharacters!!!InHere@@"

world.log << ckey(my_evil_key) // w0wigotlotsoffuncharactersinhere@@
```
