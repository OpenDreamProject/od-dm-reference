+++
title = "%%"

[extra]
usage = "x %% y"
return = "The remainder of x divided by y"
+++

This is used to calculate fractional modulo calculations, unlike the [% operator](@/language/operators/integer-modulo.md) which only works on integers. It is equivalent to `y * fract(x / y)`.
