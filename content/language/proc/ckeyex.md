+++
title = "ckeyEx"
slug = "ckeyEx" # AUTOGEN FIELD
[extra]
return_type = "null, text" # AUTOGEN FIELD
is_override = false # AUTOGEN FIELD
[[extra.args]]
name = "Text" # AUTOGEN STATIC
+++

This is similar to [ckey()](@/language/proc/ckey.md), however, case is preserved, along with `-` and `_` characters. All other special characters (apart from the `@` symbol, as with `ckey()`) will be removed.

```dm
world.log << ckeyEx("OhSoWe'ReKeeping!MyCapitalLetters?Now") // OhSoWeReKeepingMyCapitalLettersNow
```
