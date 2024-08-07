+++
title = "ODHotReloadResource"
slug = "ODHotReloadResource" # AUTOGEN FIELD
[extra]
is_override = false # AUTOGEN FIELD
[[extra.args]]
name = "file_name" # AUTOGEN STATIC
type = "text" # AUTOGEN SKIP
description = "The absolute or relative path of a resource (DMI or sound file)"
+++

This causes a resource (DMI or sound files) to be reloaded from disk and re-sent to all clients who have the old version. This allows developers to try multiple sprites or sounds without needing to recompile every time, as well as publish updates of visual and audio elements while the server is running.
Alternatively you can use the server command `hotreloadresource <file path>`.
