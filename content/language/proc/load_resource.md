+++
title = "load_resource"
[extra]
od_unimplemented = true # AUTOGEN FIELD
format = [
  [
    { name = "File" },
  ],
  [
    { name = "File", description = "A resource: a sound or image." },
    { name = "KeepTime", description = "How long, in seconds, this should be loaded for." },
  ],
  [
    { name = "File1" },
    { name = "File2" },
    { name = "..." },
    { name = "KeepTime1", description = "Only applies to the previous files specified." },
    { name = "File3" },
    { name = "File4" },
    { name = "..." },
    { name = "KeepTime2", description = "Only applies to the files specified between this argument and KeepTime1" },
  ],
]
[[extra.args]]
name = "File" # AUTOGEN STATIC
+++

Combined with the [<< output](@/language/operators/lessless/output.md) operator, this informs the client to load the specified resource. If no KeepTime is specified, it is kept for the default duration. If `-1` is specified, it will attempt to be loaded indefinitely.

```dm
/client/New()
    src << load_resource('ambient.ogg', -1, 'login_music', 120)
    // keep the ambient music forever, the login music for the next 120 seconds
```
