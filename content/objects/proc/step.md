+++
title = "step"
[extra]
return_type = "num" # AUTOGEN FIELD
return_type_desc = "Returns 1 on success (successfully moved) and 0 on failure (was blocked by something)."
is_override = false # AUTOGEN FIELD
[[extra.args]]
name = "Ref" # AUTOGEN STATIC
type = "/atom/movable" # AUTOGEN FIELD
description = "The thing to be moved."
[[extra.args]]
name = "Dir" # AUTOGEN STATIC
description = "The direction to attempt to move it in, must be one of the 8 valid directions (NORTH, SOUTH, EAST, WEST, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST)."
[[extra.args]]
name = "Speed" # AUTOGEN STATIC
description = "The speed to move, in pixels."
od_unimplemented = true
+++
Moves Ref in direction Dir, respecting collision defined by {{ atom(var="density" )}} and {{ atom(proc="Cross") }}.