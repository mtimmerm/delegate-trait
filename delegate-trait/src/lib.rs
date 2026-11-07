//! Lets library authors provide downstream crates with automatic implementations
//! of (possibly many) traits through a single "delegate" trait.
//!
//! # Example
//!
//! A library author provides some traits that are expected to be commonly
//! implemented, with blanket implementations that forward to a
//! [`DelegateProvider`]'s target:
//!
//! ```
//! use delegate_trait::DelegateProvider;
//!
//! pub trait Surface {
//!     fn area(&self) -> f64;
//! }
//!
//! pub trait Body {
//!     fn volume(&self) -> f64;
//! }
//!
//! impl<T> Surface for T
//! where
//!     T: DelegateProvider,
//!     T::Target: Surface,
//! {
//!     #[inline]
//!     fn area(&self) -> f64 {
//!         self.delegate_ref().area()
//!     }
//! }
//!
//! impl<T> Body for T
//! where
//!     T: DelegateProvider,
//!     T::Target: Body,
//! {
//!     #[inline]
//!     fn volume(&self) -> f64 {
//!         self.delegate_ref().volume()
//!     }
//! }
//! ```
//!
//! The same crate, or a different one, then declares a delegate trait with
//! [`#[delegate_trait(...)]`](delegate_trait), and chooses which of those traits
//! it provides by implementing them for the generated marker subtrait:
//!
//! ```
//! # use delegate_trait::DelegateProvider;
//! # pub trait Surface { fn area(&self) -> f64; }
//! # pub trait Body { fn volume(&self) -> f64; }
//! use delegate_trait::delegate_trait;
//!
//! #[delegate_trait(CubicDelegate)]
//! pub trait Cubic {
//!     fn side(&self) -> f64;
//!     fn surface_area(&self) -> f64 {
//!         let s = self.side();
//!         s * s * 6.0
//!     }
//!     fn body_volume(&self) -> f64 {
//!         let s = self.side();
//!         s * s * s
//!     }
//! }
//!
//! impl Surface for dyn CubicDelegate {
//!     #[inline]
//!     fn area(&self) -> f64 {
//!         self.surface_area()
//!     }
//! }
//!
//! impl Body for dyn CubicDelegate {
//!     #[inline]
//!     fn volume(&self) -> f64 {
//!         self.body_volume()
//!     }
//! }
//! ```
//!
//! On a trait, `#[delegate_trait(CubicDelegate)]` just declares an empty subtrait,
//! `pub trait CubicDelegate: Cubic {}`. It's a separate trait for use in
//! `DelegateProvider`, to avoid conflicts; there's no reason to use a
//! `dyn CubicDelegate` for anything else.
//!
//! Now any type can pick up the common traits for free:
//!
//! ```
//! # use delegate_trait::DelegateProvider;
//! # pub trait Surface { fn area(&self) -> f64; }
//! # pub trait Body { fn volume(&self) -> f64; }
//! # impl<T> Surface for T where T: DelegateProvider, T::Target: Surface {
//! #     fn area(&self) -> f64 { self.delegate_ref().area() }
//! # }
//! # impl<T> Body for T where T: DelegateProvider, T::Target: Body {
//! #     fn volume(&self) -> f64 { self.delegate_ref().volume() }
//! # }
//! # #[delegate_trait::delegate_trait(CubicDelegate)]
//! # pub trait Cubic {
//! #     fn side(&self) -> f64;
//! #     fn surface_area(&self) -> f64 { let s = self.side(); s * s * 6.0 }
//! #     fn body_volume(&self) -> f64 { let s = self.side(); s * s * s }
//! # }
//! # impl Surface for dyn CubicDelegate { fn area(&self) -> f64 { self.surface_area() } }
//! # impl Body for dyn CubicDelegate { fn volume(&self) -> f64 { self.body_volume() } }
//! use delegate_trait::delegate_trait;
//!
//! #[delegate_trait(CubicDelegate)]
//! pub struct UnitCube {}
//!
//! impl Cubic for UnitCube {
//!     fn side(&self) -> f64 {
//!         1.0
//!     }
//! }
//!
//! fn get_unit_cube_surface() -> impl Surface {
//!     // UnitCube implements Surface for free
//!     UnitCube {}
//! }
//!
//! fn get_unit_cube_body() -> impl Body {
//!     // UnitCube implements Body for free
//!     UnitCube {}
//! }
//!
//! assert_eq!(get_unit_cube_surface().area(), 6.0);
//! assert_eq!(get_unit_cube_body().volume(), 1.0);
//! ```
//!
//! On a struct, enum, or union, `#[delegate_trait(CubicDelegate)]` implements the
//! `CubicDelegate` marker subtrait for the type, as well as
//! `DelegateProvider<Target = dyn CubicDelegate>`, which lets the blanket
//! implementations of `Surface` and `Body` apply. Since `Cubic` is a supertrait of
//! `CubicDelegate`, the type must implement `Cubic`.

pub use delegate_trait_macros::delegate_trait;

/// Implementing DelegateProvider for a type enables blanket implementations of
/// traits that `Self::Target` implements.
///
/// This is usually implemented with the [`delegate_trait`] attribute, which sets
/// `Target` to `dyn` of a delegate trait's marker subtrait.
pub trait DelegateProvider: Sized {
    /// The type that blanket implementations delegate to, usually a `dyn`
    /// marker subtrait that this type implements.
    type Target: ?Sized;

    /// Get a reference to the delegate target for this value.
    fn delegate_ref<'a>(&'a self) -> &'a Self::Target;
}
