+++
title = "Comments"
+++

Sometimes extra explanation is warranted for a section of code.
Programmers can leave *comments* in their source code that the compiler will ignore but people reading the source code may find useful.

Here’s a simple (line) comment. These last until the end of the line.
```dm
// Hello World!
```

There are also multi-line, or block comments. These last until you close them with the closing delimiter.
They can also be nested.
```dm
/*
 * This is another type of comment, a block comment.
 * Block comments are useful for temporarily disabling chunks of code.
 * /* Block comments can be /* nested, */ */ so it takes only a few
 * keystrokes to comment out large chunks of code.
 */

/*
You don't need the `*` on every line either,
it just looks nice.
*/
```
