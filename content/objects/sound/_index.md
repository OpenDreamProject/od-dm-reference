+++
title = "/sound"
template = "object.html"
weight = 19

[extra]
parent_type = "/datum"
+++

`/sound` datums represent sounds. Instantiating them is very similar to lists:
```dm
var/sound/woof = sound('woof.ogg') // generally preferred
// OR
var/sound/bark = new/sound('bark.ogg')
```

You can change various variables on this object to control how a sound will play.
The most basic way to make a sound play is to send it directly to a {{ client() }} or {{ mob() }}:
```dm
var/sound/boink = sound('boink.ogg')
client << boink
```
