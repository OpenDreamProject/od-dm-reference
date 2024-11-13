+++
title = "/exception"
template = "object.html"
weight = 7

[extra]
parent_type = "/datum"
+++

When a runtime error occurs and {{ world(proc="Error") }} is defined, the first argument will be a populated exception datum.

When the EXCEPTION macro is used, a new exception will be generated. This can be used by a throw statement within a try/catch block, and the thrown exception will be caught in the catch statement.

{% parity() %}
In try/catch blocks, if `throw EXCEPTION` is not used, no exception datum will be generated. Instead, the first argument to the catch block will be the text of the exception.
{% end %}