+++
title = "link"
[extra]
od_unimplemented = true # AUTOGEN FIELD
[[extra.args]]
name = "url" # AUTOGEN STATIC
description = "Where the user should be connected to."
+++

This is combined with the [<< output](@/language/operators/lessless/output.md) operator to send a link to a client. This will open in the user's browser.

{% unsupported() %}
In BYOND, a BYOND server address can be provided. This will disconnect and reconnect the user to the provided world, eg:

```dm
user << link("byond://address:port")
```

{% end %}
