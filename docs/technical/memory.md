# Memory
This document outlines the memory model for synth

# Goals
The goals of synth are for frictionless, safe(ish) systems development.

## Safety
It should be safe without being annoying to program in (cough cough Rust). The compiler should work to make the programmer's life easier, not the other way around.

# Proposed Solution
## Functional Requirements
- A 'resource' being able to have multiple live 'owners' (i.e. multiple data structures & control flows can 'use' the same memory)
- Thread safety (we can safely lock and manage memory from multiple execution contexts) 

## How it works
- A value which can be 'moved' is called a 'link' (i.e. a function return value, a variable assignment etc). we want each link to propogate its value to its owners so the owners always have an up-to-date view of the value.
- Each link target (i.e. say i have some memory which is pointed to by some other data) must remain alive until its no longer used
- Each link can specify how/where its allocated
- Each link can specify how it wants to 'use' the resource
```
x: u32 = 123

y : u32 = x
z : u32 = x

x++ // here x and y will not update as the type is non-linked


y : link(u32) = link(x)
z : link(u32) = link(x)

x++ // here y and z will be updated as they are links to x

free(x) // this will fail as y and z are still linked

```


# Resources
- https://jondgoodwin.com/pling/gmm.pdf
- https://tutorial.ponylang.io/gotchas/garbage-collection.html
- https://www.ponylang.io/media/papers/OGC.pdf