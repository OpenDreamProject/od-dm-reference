+++
title = "issaved"
[extra]
od_unimplemented = true # AUTOGEN FIELD
[[extra.args]]
name = "v" # AUTOGEN STATIC
description = "The variable to test"
+++

A variable that can be saved is any variable that is not declared as `/global`, `/const`, or `/tmp`.

```dm
/datum/a
    var/tmp/foo = "aaa"
    var/bar = "bbb"

/proc/main()
    var/datum/a/thing = new

    world.log << issaved(thing.foo) // 0
    world.log << issaved(thing.bar) // 1
```
