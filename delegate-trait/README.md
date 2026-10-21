# delegate-trait

Delegation support for blanket trait implementations.

A type implements `DelegateProvider` to expose a `DelegateRef` to some target,
usually a `dyn Trait` view of itself. Library authors can then write blanket
implementations of their traits for every `T: DelegateProvider` whose
`DelegateRef<'_, T::Target>` implements a suitable delegate trait.

The `#[delegate_trait(Trait)]` attribute implements
`DelegateProvider<Target = dyn Trait>` for a struct, enum, or union:

```rust
use delegate_trait::delegate_trait;

trait Shape {
    fn area(&self) -> f64;
}

#[delegate_trait(Shape)]
struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}
```

See the [crate documentation](https://docs.rs/delegate-trait) for a complete example.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
