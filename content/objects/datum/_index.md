+++
title = "/datum"
template = "object.html"
weight = 5
+++

Datum is the parent of most types in DreamMaker - and as such, the variables and procedures defined on it are inherited by most objects. It is wise to be careful when defining new variables on datum, and when defining procedures directly on datum - as this code will be run extremely frequently in more complex games.

You can define types without specifying /datum, but these types parent type will still be /datum.

```dm
/datum/my_new_type

/my_new_type

/world/New()
    world.log << /datum/my_new_type::parent_type // /datum
    world.log << /my_new_type::parent_type // /datum
```

Note that, despite this sharing the same name and inheritance structure, they are still distinct types. Vars and procedures defined on `/datum/my_new_type` are not shared on `/my_new_type`.