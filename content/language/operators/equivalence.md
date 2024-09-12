+++
title = "~="

[extra]
usage = "x ~= y"
return = "1 if x and y are equivalent, 0 otherwise"
+++

Equivalence is less strict than equality, and these are equivalent, when they would not be equal:

- two different {{ list() }}s with the same contents,
- two different {{ matrix() }} with the same values,

For strict equality, use the [==](@/language/operators/equality.md) operator.
