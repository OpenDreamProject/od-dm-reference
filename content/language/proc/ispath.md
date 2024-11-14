+++
title = "ispath"
[extra]
format = [
    [{name = "Val"}],
    [{name = "Val", description = "A type path."}, {name = "Type", description = "A type path or instance."}]
]
[[extra.args]]
name = "Val" # AUTOGEN STATIC
[[extra.args]]
name = "Type" # AUTOGEN STATIC
[extra.return]
type = "num" # AUTOGEN FIELD
description = """

If Type is provided: 1 if Val is a typepath derived from Type; 0 if it is not.
If Type is not provided, 1 if Val is a typepath; 0 if is not.
"""
+++

```dm
var/what_thing = /mob/player

if(ispath(what_thing))
    world.log << "it's a path..."

if(ispath(what_thing, /mob))
    world.log << "it's some kind of mob..."

if(ispath(what_thing, /mob/player))
    world.log << "it's a player!"
```
