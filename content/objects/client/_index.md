+++
title = "/client"
template = "object.html"
weight = 3

+++

Every player has a client object representing their connection to the game. In turn, every client has a {{ mob() }}, representing the player physically on the map - this is on the {{ client(var="mob") }}. If this, the {{ mob(var="client") }} or the {{ mob(var="key" )}} is reassigned, the client will be switched to the new mob.

```dm
// when this object is clicked
/obj/body_swapper/Click() 
    // create a new /mob
    var/mob/new_body = new(loc) 

    // and reassign our clickers client to the new body
    new_body.client = = usr.client 
    // this could also be represented as
    usr.client.mob = new_body
    // or
    new_body.key = usr.key
```

When this operation occurs, {{ mob(proc="Logout") }} will be called on the old mob, and {{ mob(proc="Login") }} will be called on the new mob.