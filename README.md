dyn_compatible
===

Dyn compatible marker.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides marker to indicate wheather the trait is [`dyn` compatible]
or not. ("`dyn` compatible" is also called "object safe".)

## Background

It is important if trait is `dyn` compatible or not. However, Rust syntax does
not provide marker for it. Instead, it is determined from the kinds of methods
included in the trait. So, at code reading, we can't judge that without seeing
all methods of the trait. Also, careless methods adding and removing to traits
may result in unexpected `dyn` compatibility change.

## Examples

```rust
#[dyn_compatible(true)]
trait MyTrait {
    fn some_method(&self);
}
```

## Under the hood

For example, following two traits are ...

```rust
#[dyn_compatible(true)]
trait DynTrait {
    fn some_method(&self);
}

#[dyn_compatible(false)]
trait NotDynTrait {
    fn some_method(&self);
}
```

... expanded like these.

```rust
trait DynTrait {
    fn some_method(&self);
}

impl dyn DynTrait {}

trait NotDynTrait: dyn_compatible::NotDyn {
    fn some_method(&self);
}
```

## Versions

See the [CHANGELOG](CHANGELOG.md).
