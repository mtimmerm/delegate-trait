//! Delegation support for blanket trait implementations.
//!
//! A type implements [`DelegateProvider`] to expose a [`DelegateRef`] to some
//! target, usually a `dyn Trait` view of itself.  Library authors can then write
//! blanket implementations of their traits for every `T: DelegateProvider`
//! whose `DelegateRef<'_, T::Target>` implements a suitable delegate trait.
//!
//! The [`delegate_trait`] attribute macro implements
//! `DelegateProvider<Target = dyn Trait>` for a type.
//!
//! # Example
//!
//! ```
//! use delegate_trait::{DelegateProvider, DelegateRef, delegate_trait};
//!
//! /// The trait that users implement.
//! trait Shape {
//!     fn area(&self) -> f64;
//! }
//!
//! /// The trait that the library wants every `Shape` provider to implement.
//! trait Describe {
//!     fn describe(&self) -> String;
//! }
//!
//! /// How a `DelegateRef` to a `dyn Shape` implements `Describe`.
//! trait DescribeDelegate {
//!     fn describe(&self) -> String;
//! }
//!
//! impl DescribeDelegate for DelegateRef<'_, dyn Shape> {
//!     fn describe(&self) -> String {
//!         format!("a shape with area {}", self.target.area())
//!     }
//! }
//!
//! impl<T> Describe for T
//! where
//!     T: DelegateProvider,
//!     for<'a> DelegateRef<'a, T::Target>: DescribeDelegate,
//! {
//!     fn describe(&self) -> String {
//!         self.delegate_ref().describe()
//!     }
//! }
//!
//! #[delegate_trait(Shape)]
//! struct Square(f64);
//!
//! impl Shape for Square {
//!     fn area(&self) -> f64 {
//!         self.0 * self.0
//!     }
//! }
//!
//! assert_eq!(Square(2.0).describe(), "a shape with area 4");
//! ```

use core::ops::Deref;

pub use delegate_trait_macros::delegate_trait;

/// Implementing DelegateProvider for a struct enables blanket implementations of traits
/// for which `DelegateRef<'_, Self::Target>` implements a suitable delegate.
pub trait DelegateProvider {
    /// `delegate_ref()` returns a wrapped reference to this type.
    type Target: ?Sized;

    /// Get the delegate implementation (usually some kind of reference)
    /// for the provider type.
    fn delegate_ref<'a>(&'a self) -> DelegateRef<'a, Self::Target>;
}

/// A wrapped reference to a delegate target, returned by
/// [`DelegateProvider::delegate_ref`].
pub struct DelegateRef<'a, T: ?Sized> {
    /// The delegate target.
    pub target: &'a T,
}

impl<T: ?Sized> DelegateRef<'_, T> {
    /// Wrap a reference to a delegate target.
    pub fn new<'a>(target: &'a T) -> DelegateRef<'a, T> {
        DelegateRef { target }
    }
}

impl<T: ?Sized> Deref for DelegateRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.target
    }
}
