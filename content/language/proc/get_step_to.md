+++
title = "get_step_to"
[extra]
od_unimplemented = true # AUTOGEN FIELD
[[extra.args]]
name = "Ref" # AUTOGEN STATIC
description = "The moving object."
[[extra.args]]
name = "Trg" # AUTOGEN STATIC
description = "The object being moved away towards."
[[extra.args]]
name = "Min" # AUTOGEN STATIC
default_value = "0" # AUTOGEN FIELD
description = "If the two objects are within this many steps, no step is calculated."
[extra.return]
type = "/turf"
description = "The location that should be moved to; 0 if no path is calculated."
+++
