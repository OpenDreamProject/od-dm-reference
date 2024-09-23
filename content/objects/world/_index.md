+++
title = "/world"
template = "object.html"
weight = 21
+++

The world data structure is used to contain vars that indicate the state of the game world, and procedures that can be overriden to handle specific events, such as {{ world(proc="Reboot") }} and {{ world(proc="New") }}. World is globally accessible as `world`, but cannot be extended with new global variables.
