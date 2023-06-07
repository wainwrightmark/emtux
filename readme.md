## Emtux

Concurrent collections and primitives that are impossible to deadlock if used correctly.

Exempt introduces several types (with more on the way) that allow for concurrent mutable access to multiple resources without the risk of deadlocking. This is done by using the rust ownership system to ensure that locks are always taken in the same order, thus preventing the possibility of deadlock.

Consider the following example of swapping values in a `vec`.

If these tasks were interleaved in exactly the wrong way, a deadlock would occur.

With `exempt`, the code would be written like this
