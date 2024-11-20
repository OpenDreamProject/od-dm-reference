+++
title = "json_encode"
[[extra.args]]
name = "Value" # AUTOGEN STATIC
description = "The DreamMaker value to encode as a JSON string."
[[extra.args]]
name = "flags" # AUTOGEN STATIC
description = "Optional, additional flags to change how the JSON is encoded."
+++

For both {{ list() }}s and {{ matrix() }} objects that are provided as Value, they will be encoded as a JSON array, ie `[1,0,0,0,1,0]`. An [associated list](@/objects/list/association.md) will be encoded as a JSON object.

Any lists included in both normal and associated lists will also be encoded.

If a datum is provided, or is included in a list provided to the proc, they will not be serialized, and instead encoded the same as `"[datum]"`.

```dm
var/list/my_list = list("foo" = "bar")
my_list["do_re_mi"] = list("fa_sol_la")
my_list.Add("ti")

world.log << json_encode(my_list)
// {"foo":"bar","do_re_mi":["fa_sol_la"],"ti":null}
```

The accepted flags are:

| Flag              | Description                                                                         |
| ----------------- | :---------------------------------------------------------------------------------- |
| JSON_PRETTY_PRINT | - Spaces added after colons and commas.                                             |
|                   | - Arrays and objects with content will have line breaks and tabs added before items |
