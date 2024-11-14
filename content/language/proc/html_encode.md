+++
title = "html_encode"
[[extra.args]]
name = "PlainText" # AUTOGEN STATIC
description = "A string to escape."
[extra.return]
type = "text" # AUTOGEN FIELD
description = "The provided string, escaped."
+++

Before text can be displayed in a HTML document, certain reserved characters must be encoded, as they have special meaning in HTML. Using html_encode() on the string before it is displayed ensures that it is displayed literally.

```dm
world.log << html_encode("My & strin'g full > of reserved ' < characters \".")
// My &amp; strin&#39;g full &gt; of reserved &#39; &lt; characters &quot;.
```
