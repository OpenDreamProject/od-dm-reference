+++
title = "/atom"
template = "object.html"
weight = 2

[extra]
parent_type = "/datum"
+++

The parent of all the objects that can appear in the game world - {{ area() }}, {{ turf() }}, {{ obj() }} and {{ mob() }} (atom!). All variables and procs shared by physical objects are defined here - it does not need to be created itself. 