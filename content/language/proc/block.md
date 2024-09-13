+++
title = "block"
[extra]
return_type = "path, /list" # AUTOGEN FIELD
return_type_desc = "A list of turfs in a 3D block"
is_override = false # AUTOGEN FIELD

format = [
    [
        { name = "Start", type = "/atom", description = "The lower left corner of the block." }, 
        { name = "End", type = "/atom", description = "The upper right corner of the block." }
    ],
    [
        { name = "StartX" },
        { name = "StartY" },
        { name = "StartZ" },
        { name = "EndX" },
        { name = "EndY" },
        { name = "EndZ" },
    ]
]

[[extra.args]]
name = "Start" # AUTOGEN STATIC
type = "/atom" # AUTOGEN FIELD
[[extra.args]]
name = "End" # AUTOGEN STATIC
type = "/atom" # AUTOGEN FIELD
[[extra.args]]
name = "StartZ" # AUTOGEN STATIC
[[extra.args]]
name = "EndX" # AUTOGEN STATIC
[[extra.args]]
name = "EndY" # AUTOGEN STATIC
[[extra.args]]
name = "EndZ" # AUTOGEN STATIC
+++

This proc allows for us to get a list of the turfs within a specified area.

```dm
for(var/turf/my_turf in block(1, 1, 1, 10, 10, 10))
    my_turf.icon_state = "green"
```

This will change the {{ atom(var="icon_state") }} of every turf, beginning at the turf located at X = 1, Y = 1, Z = 1 and finishing at the turf located at X = 10, Y = 10 and Z = 10, to "green". This list is inclusive of the Start and End turfs.
