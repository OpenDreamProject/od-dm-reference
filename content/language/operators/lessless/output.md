+++
title = "output"

[extra]
usage = "x << y"
+++

This will output the value `y`, which may be a sound, image or text, to any players connected within `x`, which may be a mob, client, a list containing mobs, or the whole world.

```dm
usr << "Hello, person reading this!"
world << "Wow, that's a lot of people!"
view(usr) << "To everyone around!"

usr << sound("epic.wav")
```
