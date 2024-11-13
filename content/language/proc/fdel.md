+++
title = "fdel"
[[extra.args]]
name = "File" # AUTOGEN STATIC
description = "The file system entry to delete."
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 on success; 0 on failure"
+++

Deletes the specified entry in the file system. If the filename ends with a `/`, it will recursively delete all contents within the directory.