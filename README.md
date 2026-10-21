# delegate-trait

Delegation support for blanket trait implementations in Rust.

A type implements `DelegateProvider` to expose a `DelegateRef` to some target,
usually a `dyn Trait` view of itself. Library authors can then write blanket
implementations of their traits for every `T: DelegateProvider` whose
`DelegateRef<'_, T::Target>` implements a suitable delegate trait.
`DelegateRef<'_, T>` derefs to `T`.

The `#[delegate_trait(Trait)]` attribute implements
`DelegateProvider<Target = dyn Trait>` for a struct, enum, or union.

## Crates

| Crate | Description |
| --- | --- |
| [`delegate-trait`](delegate-trait) | `DelegateProvider`, `DelegateRef`, and a re-export of the macros. Depend on this one. |
| [`delegate-trait-macros`](delegate-trait-macros) | The `#[delegate_trait(...)]` procedural macro. |

## Example

```rust
use delegate_trait::{DelegateProvider, DelegateRef, delegate_trait};

/// The trait that users implement.
trait Shape {
    fn area(&self) -> f64;
}

/// The trait that the library wants every `Shape` provider to implement.
trait Describe {
    fn describe(&self) -> String;
}

/// How a `DelegateRef` to a `dyn Shape` implements `Describe`.
trait DescribeDelegate {
    fn describe(&self) -> String;
}

impl DescribeDelegate for DelegateRef<'_, dyn Shape> {
    fn describe(&self) -> String {
        format!("a shape with area {}", self.area())
    }
}

impl<T> Describe for T
where
    T: DelegateProvider,
    for<'a> DelegateRef<'a, T::Target>: DescribeDelegate,
{
    fn describe(&self) -> String {
        self.delegate_ref().describe()
    }
}

#[delegate_trait(Shape)]
struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

assert_eq!(Square(2.0).describe(), "a shape with area 4");
```

The attribute accepts any trait object bounds, such as
`#[delegate_trait(MyTrait<u32> + Send + Sync)]`. The annotated type must
implement the trait(s) and be `'static`.

## Development

```sh
cargo test
cargo clippy --all-targets
```

The minimum supported Rust version is 1.85.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
