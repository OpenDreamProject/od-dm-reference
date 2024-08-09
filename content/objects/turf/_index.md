+++
title = "/turf"
template = "object.html"
weight = 20

[extra]
parent_type = "/atom"
+++

Turfs are the tiles that make up the world. These are immovable, deriving from {{ atom() }}. Turfs can be replaced by creating a new turf, providing the old one as the loc argument.

```dm
/turf/floor
    desc = "The floor. You can walk on this."

/turf/wall
    desc = "The wall. You can't walk through this."
    density = TRUE
```