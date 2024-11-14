+++
title = "isturf"
[extra]
format = [
  [
    { name = "Loc1", description = "Any number of objects to test." },
    { name = "Loc2" },
    { name = "..." },
  ],
]
[[extra.args]]
name = "Loc1" # AUTOGEN STATIC
[extra.return]
type = "num" # AUTOGEN FIELD
description = "1 if all the provided arguments are {{ turf() }}s; 0 if any are not."
+++
