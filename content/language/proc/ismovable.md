+++
title = "ismovable"
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
description = "1 if all provided arguments are valid movable atoms; 0 if any are not."
+++

All children of {{ atom_movable() }} pass this test, including user defined children, so is similar, but distinct to `isobj(loc) || ismob(loc)`.
