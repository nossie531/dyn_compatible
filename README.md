dyn_compatible
===

Dyn compatible marker.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides marker whether trait is [dyn compatible] or not.
("dyn compatible" is also called "object safe".)

## Background

It is important if trait is dyn compatible or not. However, Rust syntax does
not provide marker for it. Instead, it is determined from the kinds of methods
included in the trait. So, at code reading, we can't judge that without seeing
all methods of the trait. Also, careless methods adding and removing to traits
may result in unexpected dyn compatibility change.

## Examples

```
#[dyn_compatible(true)]
pub trait MyTrait {
    fn some_method(&self);
}
```

<!-- Links -->
[dyn compatible]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
