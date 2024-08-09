+++
title = "/area"
template = "object.html"
weight = 1

[extra]
parent_type = "/atom"
+++

Regions of tiles on the map are assigned to an area. Each area can be entered and exited by movable atoms as they move across tiles. As you cross a {{ turf() }} and enter a different area, {{ turf(proc="Entered") }} is called on the turf and {{ atom(proc="Entered") }} is called on the area.

```dm
/area
    name = "Somewhere Unknown"

/area/Entered(atom/movable/entering_object) // called when something (an object or mob) moves into this area
    entering_object << "You've just entered [name]" // we send the name of the area to the object. if this is not a mob, the operation is ignored

/area/entrance_way
    name = "The Entrance"