+++
title = "gradient"
[extra]
format = [
  [
    { name = "Item1", description = "Elements of a color gradient list." },
    { name = "Item2" },
    { name = "..." },
    { name = "index" },
  ],
  [
    { name = "Gradient", type = "/list", description = "A color gradient list." },
    { name = "index", description = "The index on the gradient to perform the interpolation on." },
  ],
]
return = { type = "text", description = "A color, either in #rrggbb or #rrggbbaa format." }
[[extra.args]]
name = "A" # AUTOGEN STATIC
[[extra.args]]
name = "index" # AUTOGEN STATIC
+++

Generates a gradient between the specified elements, and retrieves the color at the specified point along the gradient, per the index argument.

```dm
world.log << gradient("#FF0000", "#000000", 0.5) // outputs #7F0000, as we have halved the red value
```
