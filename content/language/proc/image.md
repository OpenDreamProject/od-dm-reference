+++
title = "image"
[[extra.args]]
name = "icon" # AUTOGEN STATIC
description = "An icon, object type, object instance or image."
[[extra.args]]
name = "loc" # AUTOGEN STATIC
description = "Optional, the location to display the image."
[[extra.args]]
name = "icon_state" # AUTOGEN STATIC
description = "Optional, the icon state to use."
[[extra.args]]
name = "layer" # AUTOGEN STATIC
default_value = "FLY_LAYER for icons, otherwise inherited from the object"
description = "Optional, the layer this should appear on."
[[extra.args]]
name = "dir" # AUTOGEN STATIC
description = "Optional, the direction to use."
[[extra.args]]
name = "pixel_x" # AUTOGEN STATIC
[[extra.args]]
name = "pixel_y" # AUTOGEN STATIC
+++

This constructs a new {{ image() }}, similar to using `new /image()`.

```dm
/obj/hat_giver/Clicked()
    usr.overlays += image('hat.dmi', "big_hat")
```

The arguments accepted by the proc can be used to override the values inherited from the object, if passed an object in the first argument.

The image is attached to the specified loc, following movable atoms, and can be removed by reassigning the {{ image(var="loc") }} or deleting the image.
