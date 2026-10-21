//! Procedural macros for the [`delegate-trait`](https://docs.rs/delegate-trait) crate.
//!
//! Use these macros through the `delegate-trait` crate, which re-exports them.
//! The generated code refers to items in `::delegate_trait`.

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Token, TypeParamBound, parse_macro_input, parse_quote};

/// Implements `DelegateProvider<Target = dyn Trait>` for a struct, enum, or union.
///
/// ```ignore
/// #[delegate_trait(MyTrait)]
/// struct MyType { /* ... */ }
/// ```
///
/// expands to the item unchanged, plus:
///
/// ```ignore
/// impl ::delegate_trait::DelegateProvider for MyType
/// where
///     Self: MyTrait + 'static,
/// {
///     type Target = dyn MyTrait;
///
///     fn delegate_ref<'a>(&'a self) -> ::delegate_trait::DelegateRef<'a, Self::Target> {
///         ::delegate_trait::DelegateRef::new(self)
///     }
/// }
/// ```
///
/// The argument may be any list of trait object bounds, e.g.
/// `#[delegate_trait(MyTrait<u32> + Send + Sync)]`.
///
/// The type must implement the trait(s) and be `'static`, because the
/// delegate target is `dyn Trait + 'static`.  For generic types these
/// requirements are expressed as `where` bounds on the generated impl.
#[proc_macro_attribute]
pub fn delegate_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let bounds =
        parse_macro_input!(attr with Punctuated::<TypeParamBound, Token![+]>::parse_terminated);
    let input = parse_macro_input!(item as DeriveInput);

    if bounds.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected a trait, e.g. `#[delegate_trait(MyTrait)]`",
        )
        .to_compile_error()
        .into();
    }

    let name = &input.ident;
    let mut generics = input.generics.clone();
    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: #bounds + 'static));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #input

        impl #impl_generics ::delegate_trait::DelegateProvider for #name #ty_generics
        #where_clause
        {
            type Target = dyn #bounds;

            #[inline]
            fn delegate_ref<'a>(&'a self) -> ::delegate_trait::DelegateRef<'a, Self::Target> {
                ::delegate_trait::DelegateRef::new(self)
            }
        }
    }
    .into()
}
