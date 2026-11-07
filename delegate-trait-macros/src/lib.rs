//! Procedural macros for the [`delegate-trait`](https://docs.rs/delegate-trait) crate.
//!
//! Use these macros through the `delegate-trait` crate, which re-exports them.
//! The generated code refers to items in `::delegate_trait`.

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Item, Token, TypeParamBound, parse_macro_input, parse_quote};

/// Declares the marker subtrait for a delegate trait, or opts a type into one.
///
/// # On a trait
///
/// ```ignore
/// #[delegate_trait(CubicDelegate)]
/// pub trait Cubic { /* ... */ }
/// ```
///
/// expands to the trait unchanged, plus an empty marker subtrait with the same
/// visibility, generic parameters, and `where` clause:
///
/// ```ignore
/// pub trait CubicDelegate: Cubic {}
/// ```
///
/// Implement traits that have `DelegateProvider` blanket implementations for
/// `dyn CubicDelegate` to choose which of them types with this delegate get.
///
/// # On a struct, enum, or union
///
/// ```ignore
/// #[delegate_trait(CubicDelegate)]
/// pub struct UnitCube { /* ... */ }
/// ```
///
/// expands to the item unchanged, plus:
///
/// ```ignore
/// impl CubicDelegate for UnitCube {}
///
/// impl ::delegate_trait::DelegateProvider for UnitCube
/// where
///     Self: CubicDelegate + 'static,
/// {
///     type Target = dyn CubicDelegate;
///
///     fn delegate_ref<'a>(&'a self) -> &'a Self::Target {
///         self
///     }
/// }
/// ```
///
/// The type must implement the delegate trait (`Cubic` above), which is a
/// supertrait of the marker.
///
/// The argument may include extra trait object bounds, e.g.
/// `#[delegate_trait(CubicDelegate + Send + Sync)]`.  Only the first trait
/// is implemented for the type; all of the bounds go into the `dyn` target,
/// so blanket implementations only apply through traits implemented for
/// `dyn CubicDelegate + Send + Sync`.
///
/// The type must be `'static`, because the delegate target is
/// `dyn CubicDelegate + 'static`.  For generic types this requirement is
/// expressed as a `where` bound on the generated impl.
#[proc_macro_attribute]
pub fn delegate_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let bounds =
        parse_macro_input!(attr with Punctuated::<TypeParamBound, Token![+]>::parse_terminated);
    let item = parse_macro_input!(item as Item);

    if bounds.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected a trait, e.g. `#[delegate_trait(MyTrait)]`",
        )
        .to_compile_error()
        .into();
    }

    if let Item::Trait(it) = &item {
        // On a trait, the argument is the name of the new trait to declare.
        let new_name = match bounds.first() {
            Some(TypeParamBound::Trait(tb)) if bounds.len() == 1 => tb.path.get_ident(),
            _ => None,
        };
        let Some(new_name) = new_name else {
            return syn::Error::new_spanned(
                &bounds,
                "on a trait, expected the name of the new trait, e.g. `#[delegate_trait(MyTrait)]`",
            )
            .to_compile_error()
            .into();
        };
        let name = &it.ident;
        let generics = &it.generics;
        let (_, ty_generics, where_clause) = generics.split_for_impl();
        let vis = &it.vis;
        quote! {
            #item
            /// delegate trait for [`#name`]
            #vis trait #new_name #generics : #name #ty_generics
            #where_clause
            {}
        }
        .into()
    } else {
        let (name, generics) = match &item {
            Item::Struct(it) => (it.ident.clone(), &it.generics),
            Item::Enum(it) => (it.ident.clone(), &it.generics),
            Item::Union(it) => (it.ident.clone(), &it.generics),
            _ => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "expected a trait, struct, enum, or union",
                )
                .to_compile_error()
                .into();
            }
        };
        // The trait from the parameter is the first trait bound; any others
        // (e.g. `Send + Sync`) only apply to the `dyn` target.
        let Some(trait_path) = bounds.iter().find_map(|b| match b {
            TypeParamBound::Trait(tb) => Some(&tb.path),
            _ => None,
        }) else {
            return syn::Error::new_spanned(&bounds, "expected a trait")
                .to_compile_error()
                .into();
        };
        let (trait_impl_generics, trait_ty_generics, trait_where_clause) =
            generics.split_for_impl();

        let mut generics = generics.clone();
        generics
            .make_where_clause()
            .predicates
            .push(parse_quote!(Self: #bounds + 'static));
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        quote! {
            #item

            impl #trait_impl_generics #trait_path for #name #trait_ty_generics
            #trait_where_clause
            {}

            impl #impl_generics ::delegate_trait::DelegateProvider for #name #ty_generics
            #where_clause
            {
                type Target = dyn #bounds;

                #[inline]
                fn delegate_ref<'a>(&'a self) -> &'a Self::Target {
                    self
                }
            }
        }
        .into()
    }
}
