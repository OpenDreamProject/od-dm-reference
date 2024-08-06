+++
title = "step"
[extra]
return_type = "num"
return_type_desc = "Returns 1 on success (successfully moved) and 0 on failure (was blocked by something)."
is_override = false
[[extra.args]]
name = "Ref"
type = "/atom/movable"
description = "The thing to be moved."
[[extra.args]]
name = "Dir"
description = "The direction to attempt to move it in, must be one of the 8 valid directions (NORTH, SOUTH, EAST, WEST, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST)."
[[extra.args]]
name = "Speed"
description = "The speed to move, in pixels."
od_unimplemented = true
+++
Moves Ref in direction Dir, respecting collision defined by [density](@/objects/atom/var/density.md) and [Cross](@/objects/atom/proc/cross.md).