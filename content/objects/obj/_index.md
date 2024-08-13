+++
title = "/obj"
template = "object.html"
weight = 15

[extra]
parent_type = "/atom/movable"
+++

Objs are (typically) for most non-creature ({{ mob() }}) and non-wall/floor ({{ turf() }}) objects in the game world.

```dm
/obj/cake
    name = "tasty cake"
    desc = "Wow, a yummy cake!
```