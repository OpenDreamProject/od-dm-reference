+++
title = "get_dist"
[[extra.args]]
name = "Loc1" # AUTOGEN STATIC
description = "An object on the map."
type = "/atom" # AUTOGEN FIELD
[[extra.args]]
name = "Loc2" # AUTOGEN STATIC
description = "An object on the map."
type = "/atom" # AUTOGEN FIELD
[extra.return]
type = "num" # AUTOGEN FIELD
description = "The number of tiles between both arguments."
+++

If either object is not on the map, 127 will be returned.
If both arguments are the same object, -1 will be returned.

{% parity() %}
In BYOND, the distance in Z level is also calculated.
{% end %}
