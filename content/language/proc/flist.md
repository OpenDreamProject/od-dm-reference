+++
title = "flist"
[[extra.args]]
name = "Path" # AUTOGEN STATIC
description = "The path in the file system to return the contents of."
[extra.return]
type = "/list" # AUTOGEN FIELD
description = "A list of files and directories in the specified path."
+++

This is non-recursive, and only returns the contents of the directory specified. Names returned are relative to the specified path.

Path must be a directory with a trailing slash.

```dm
// given a directory structure like
//
// example
// ├── maps
// │   └── map.dmm
// ├── code.dme
// └── icons
//     ├── icon.dmi
//     └── other_icon.dmi

for(var/item in flist("icons/"))
    world.log << item // "icon.dmi", "other_icon.dmi"

for(var/item in flist("./"))
    world.log << item // "maps/", "code.dme", "icons/"

```
