//! Tests for the `delegate_trait` attribute macro.
//!
//! Library traits (`Surface`, `Body`, `Describe`) each have a single blanket
//! impl over `DelegateProvider`.  Delegate traits (`Cubic`, `Flat`, `Labeled`)
//! choose which of them to provide by implementing them for their `dyn` marker
//! subtrait.  Types marked with `#[delegate_trait(...)]` then pick up those
//! traits by implementing only the delegate trait.

use std::fmt::Display;

use delegate_trait::{DelegateProvider, delegate_trait};

//=============================================================================
// Library traits with blanket impls
//=============================================================================

trait Surface {
    fn area(&self) -> f64;
}

impl<T> Surface for T
where
    T: DelegateProvider,
    T::Target: Surface,
{
    fn area(&self) -> f64 {
        self.delegate_ref().area()
    }
}

trait Body {
    fn volume(&self) -> f64;
}

impl<T> Body for T
where
    T: DelegateProvider,
    T::Target: Body,
{
    fn volume(&self) -> f64 {
        self.delegate_ref().volume()
    }
}

trait Describe {
    fn describe(&self) -> String;
}

impl<T> Describe for T
where
    T: DelegateProvider,
    T::Target: Describe,
{
    fn describe(&self) -> String {
        self.delegate_ref().describe()
    }
}

//=============================================================================
// Delegate traits
//=============================================================================

/// Provides all three library traits.
#[delegate_trait(CubicDelegate)]
trait Cubic {
    fn side(&self) -> f64;
}

impl Surface for dyn CubicDelegate {
    fn area(&self) -> f64 {
        self.side() * self.side() * 6.0
    }
}

impl Body for dyn CubicDelegate {
    fn volume(&self) -> f64 {
        self.side() * self.side() * self.side()
    }
}

impl Describe for dyn CubicDelegate {
    fn describe(&self) -> String {
        format!("cube with side {}", self.side())
    }
}

/// Provides only `Surface`.
#[delegate_trait(FlatDelegate)]
trait Flat {
    fn flat_area(&self) -> f64;
}

impl Surface for dyn FlatDelegate {
    fn area(&self) -> f64 {
        self.flat_area()
    }
}

/// A generic delegate trait.
#[delegate_trait(LabeledDelegate)]
trait Labeled<T: Display> {
    fn label(&self) -> T;
}

impl<T: Display + 'static> Describe for dyn LabeledDelegate<T> {
    fn describe(&self) -> String {
        format!("labeled {}", self.label())
    }
}

//=============================================================================
// Types that use the delegate traits
//=============================================================================

#[delegate_trait(CubicDelegate)]
struct UnitCube;

impl Cubic for UnitCube {
    fn side(&self) -> f64 {
        1.0
    }
}

#[delegate_trait(CubicDelegate)]
enum Die {
    Small,
    Large,
}

impl Cubic for Die {
    fn side(&self) -> f64 {
        match self {
            Die::Small => 1.0,
            Die::Large => 2.0,
        }
    }
}

#[delegate_trait(FlatDelegate)]
struct Square(f64);

impl Flat for Square {
    fn flat_area(&self) -> f64 {
        self.0 * self.0
    }
}

#[delegate_trait(LabeledDelegate<T>)]
struct Tag<T: Copy + Display> {
    label: T,
}

impl<T: Copy + Display> Labeled<T> for Tag<T> {
    fn label(&self) -> T {
        self.label
    }
}

#[delegate_trait(CubicDelegate + Send + Sync)]
struct SyncCube;

impl Cubic for SyncCube {
    fn side(&self) -> f64 {
        3.0
    }
}

//=============================================================================
// Tests
//=============================================================================

#[test]
fn struct_picks_up_all_provided_traits() {
    assert_eq!(UnitCube.area(), 6.0);
    assert_eq!(UnitCube.volume(), 1.0);
    assert_eq!(UnitCube.describe(), "cube with side 1");
}

#[test]
fn enum_picks_up_all_provided_traits() {
    assert_eq!(Die::Large.area(), 24.0);
    assert_eq!(Die::Large.volume(), 8.0);
    assert_eq!(Die::Small.describe(), "cube with side 1");
}

#[test]
fn delegate_trait_chooses_provided_traits() {
    // `Flat` only provides `Surface`, so `Square` has `area()` but not
    // `volume()` or `describe()`.
    assert_eq!(Square(3.0).area(), 9.0);
}

#[test]
fn generic_delegate_trait_and_type() {
    assert_eq!(Tag { label: 7u32 }.describe(), "labeled 7");
    assert_eq!(Tag { label: "x" }.describe(), "labeled x");
}

#[test]
fn usable_as_impl_and_dyn_trait() {
    fn surface() -> impl Surface {
        UnitCube
    }
    assert_eq!(surface().area(), 6.0);

    let shapes: [&dyn Surface; 3] = [&UnitCube, &Die::Large, &Square(2.0)];
    let areas: Vec<f64> = shapes.iter().map(|s| s.area()).collect();
    assert_eq!(areas, [6.0, 24.0, 4.0]);
}

#[test]
fn delegate_ref_returns_dyn_marker() {
    let target: &dyn CubicDelegate = Die::Large.delegate_ref();
    assert_eq!(target.side(), 2.0);
}

#[test]
fn extra_bounds_go_into_target() {
    let target: &(dyn CubicDelegate + Send + Sync) = SyncCube.delegate_ref();
    assert_eq!(target.side(), 3.0);
}
