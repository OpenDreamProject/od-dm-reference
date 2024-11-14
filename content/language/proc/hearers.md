+++
title = "hearers"
[[extra.args]]
name = "Depth" # AUTOGEN STATIC
description = "The radius to check for hearers in."
[[extra.args]]
name = "Center" # AUTOGEN STATIC
description = "An object on the map to check for hearers around."
[extra.return]
type = "/list" # AUTOGEN FIELD
description = "A list of /mobs that can hear the center object."
+++

An object that can hear another object is calculated similarly to [viewers()](@/language/proc/viewers.md), but ignores darkness, as you can still hear in the dark.
