+++
title = "New"
slug = "New" # AUTOGEN FIELD
[extra]
is_override = true # AUTOGEN FIELD
[[extra.args]]
name = "filename" # AUTOGEN STATIC
description = "The sqlite file to open, optional"
+++

Creates a new database datum. If the filename argument is provided, {{ database(proc="Open") }} is automatically called.
