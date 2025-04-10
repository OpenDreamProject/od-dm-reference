+++
title = "json_decode"
[[extra.args]]
name = "JSON" # AUTOGEN STATIC
description = "The JSON string to convert."
+++

An array, like `["foo", 1, "bar", 2]` will be decoded into a {{ list() }}, e.g. `list("foo", 1, "bar"), 2`. Objects, like `{"foo": "bar", "do": "re"}`, will be decoded into an [associated list](@/objects/list/association.md), e.g. `list("foo" = "bar", "do" = "re")`. Additional arrays can be embedded into arrays or objects, and an array can hold multiple objects.

```dm
var/my_json = @{"
    {
        "some_key": 1231232,
        "some_other_key": "foo",
        "my_third_key": ["an array", "of strings", 12312, "or whatever"],
        "my_final_key": {
            "another_objects_key": "the other objects value"
        }
    }
"}

var/decoded = json_decode(my_json)
for(var/key in decoded)
world.log << key // some_key, some_other_key, my_third_key, my_final_key
```

Boolean values, such as `false` and `true`, will become `0` and `1` respectively, corresponding to the FALSE and TRUE defines.

{% parity() %}
In the BYOND implementation of DreamMaker, the "strictness" of the decoding, by default, allows for the keys in JSON values to be non-string values. This can be toggled by passing the `JSON_STRICT` flag. In OpenDream, this behavior is the default, and all keys in JSON values must be strings.
{% end %}

| Flag                | Description                                                         |
| ------------------- | :------------------------------------------------------------------ |
| JSON_STRICT         | - All strings require double quotes.                                |
|                     | - NaN and Infinity values are not permitted.                        |
|                     | - Keys must be strings.                                             |
| JSON_ALLOW_COMMENTS | - Allows for both // inline and /\* \*/ long form comments in JSON. |
|                     | - The BYOND implementation enables this by default.                 |
