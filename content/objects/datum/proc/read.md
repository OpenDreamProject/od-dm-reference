+++
title = "Read"
slug = "Read" # AUTOGEN FIELD
[[extra.args]]
name = "F" # AUTOGEN STATIC
type = "/savefile" # AUTOGEN FIELD
+++

This is called when a {{ datum() }} is read from a {{ savefile() }}. The {{ savefile() }} being read from is passed as the first argument. The return value is ignored.

Example:

```dm
//create a demonstrator datum type
/datum/test
  var/test_var = 1

  //define our demonstrator Read() proc
  Read(savefile/F)
    src.test_var = 2

//create an instance of /savefile and our /datum/test
var/savefile/my_savefile = new()
var/datum/test/T = new()

//store the test datum in the savefile
my_savefile["test"] = T

//read the test datum from the savefile. This calls /datum/test/Read()
var/datum/test/T2 = my_savefile["test"]

//test_var is now 2, because Read() was called
ASSERT(T2.test_var == 2)
```
