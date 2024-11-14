+++
title = "get_step_away"
[extra.return]
type = "/turf, num"
description = "The new position; or 0 if no movement has occured."
[[extra.args]]
name = "Ref" # AUTOGEN STATIC
type = "/atom/movable" # AUTOGEN FIELD
description = "The moving object."
[[extra.args]]
name = "Trg" # AUTOGEN STATIC
type = "/atom" # AUTOGEN FIELD
description = "The object being moved away from."
[[extra.args]]
name = "Max" # AUTOGEN STATIC
default_value = "5" # AUTOGEN FIELD
description = "The maximum distance between Ref and Trg."
+++

Calculates the step in the opposite direction from the provided object.
