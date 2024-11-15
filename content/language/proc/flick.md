+++
title = "flick"
[extra]
od_unimplemented = true # AUTOGEN FIELD
[[extra.args]]
name = "Icon" # AUTOGEN STATIC
description = "An icon file, or a named icon state."
[[extra.args]]
name = "Object" # AUTOGEN STATIC
description = "The object being changed."
+++

Temporarily replaces the {{ atom(var="icon") }}, or {{ atom(var="icon_state") }}, of the specified object for the duration of the animation. This only occurs on the client - the icon or icon_state variable is not actually altered.

```dm
/obj/button/Click()
    flick("pressed_animation", src)
    usr << "You pressed me!"
```
