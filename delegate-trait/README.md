# delegate-trait

This crate lets library authors provide downstream crates with automatic
implementations of (possibly many) traits by implementing a single
"delegate" trait.

For example, a library author could provide some traits that are expected
to be commonly implemented:

```rust
trait Surface {
  fn area(&self) -> f64;
}

trait Body {
  fn volume(&self) -> f64;
}
```
To allow these traits to be automatically implemented based on delegate
traits that they or other library authors write later, they can provide
blanket implementations like this:

```rust
use delegate_trait::DelegateProvider;

impl<T> Surface for T
where
    T: DelegateProvider,
    T::Target: Surface,
{
    #[inline]
    fn area(&self) -> f64 {
        self.delegate_ref().area()
    }
}

impl<T> Body for T
where
    T: DelegateProvider,
    T::Target: Body,
{
    #[inline]
    fn volume(&self) -> f64 {
        self.delegate_ref().volume()
    }
}
```

A possibly different crate can then create traits that can provide automatic implementations of these:

```rust
use delegate_trait::delegate_trait;

#[delegate_trait(CubicDelegate)]
pub trait Cubic {
  fn side(&self) -> f64;
  fn surface_area(&self) -> f64 {
    let s = self.side();
    s*s*6.0
  }
  fn body_volume(&self) -> f64 {
    let s = self.side();
    s*s*s
  }
}

impl Surface for dyn CubicDelegate {
  #[inline]
  fn area(&self) -> f64 {
    self.surface_area()
  }
}

impl Body for dyn CubicDelegate {
  #[inline]
  fn volume(&self) -> f64 {
    self.body_volume()
  }
}
```

When applied to a trait, as above, the `#[delegate_trait(CubicDelegate)]` attribute just
declares an empty subtrait `pub trait CubicDelegate: Cubic`.  This is provided as a separate
trait to use in `DelegateProvider` to avoid conflicts.  There is no reason to use
a `dyn CubicDelegate` for any other purpose.

Now, any type can pick up the common traits for free:

```rust
use delegate_trait::delegate_trait;

#[delegate_trait(CubicDelegate)]
pub struct UnitCube {}

impl Cubic for UnitCube {
  fn side(&self) -> f64 {
    1.0
  }
}

fn get_unit_cube_surface() -> impl Surface {
  // UnitCube implements Surface for free
  UnitCube{}
}

fn get_unit_cube_body() -> impl Body {
  // UnitCube implements Body for free
  UnitCube{}
}
```

When used on a struct, union, or enum, as above, the `#[delegate_trait(CubicDelegate)]` attribute implements
the `CubicDelegate` marker subtrait on the struct, as well as the `DelegateProvider<Target = dyn CubicDelegate>`
trait, which allows the blanket implementations of `Surface` and `Body` to apply.

Since `Cubic` is a supertrait of `CubicDelegate`, it must then be implemented for the struct.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
