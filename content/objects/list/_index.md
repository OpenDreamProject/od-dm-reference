+++
title = "/list"
template = "object.html"
weight = 11
+++

A group of objects that can be accessed either by index or key. DreamMaker lists are indexed beginning at 1 - so to access the first element, you access `list[1]`.

```dm
var/list/my_list = list("my_first_element", "my_second_element", "my_third_element")
world.log << my_list[1] // prints my_first_element
```

When you create a new list using the `list()` proc with no arguments, or `new /list()`, the length will be zero. Specifying `new /list(14)`, or by including a length in the list variable declaration, will make a list of that length.

```dm
var/list/my_short_list = list()
world.log << my_short_list.len // 0
world.log << length(my_short_list) // 0

var/list/my_long_list = new /list(14)
world.log << my_long_list.len // 14

var/list/my_other_long_list[14]
world.log << my_other_long_list.len // 14
```

Lists can be resized, which will remove elements beyond the specified length if the list is shrunk, or grow the list with null values.

```dm
var/list/my_list = list("foo", "bar", "car")
my_list.len = 2

for(var/element in my_list)
    world.log << element // "foo", "bar"

var/list/my_other_list = list("foo")
my_other_list.len = 3

for(var/element in my_list)
    world.log << isnull(element) // FALSE (0), TRUE (1), TRUE (1)
```