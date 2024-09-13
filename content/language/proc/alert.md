+++
title = "alert"
[[extra.args]]
name = "Usr" # AUTOGEN STATIC
description = "The user"
[[extra.args]]
name = "Message" # AUTOGEN STATIC
[[extra.args]]
name = "Title" # AUTOGEN STATIC
[[extra.args]]
name = "Button1" # AUTOGEN STATIC
default_value = "Ok" # AUTOGEN FIELD
[[extra.args]]
name = "Button2" # AUTOGEN STATIC
[[extra.args]]
name = "Button3" # AUTOGEN STATIC
[extra.return]
type = "text" # AUTOGEN FIELD
+++

Displays a customizable alert message to a user (either a {{ mob() }} or {{ client() }}). It accepts a title, message, and up to three buttons as text. If no buttons are specified, the first defaults to "Ok".

If no user is explicitly provided, the current user is used and arguments are shifted. The function checks for a valid connection; if none exists, it returns "OK". Otherwise, it sends the alert and returns the user's interaction.
