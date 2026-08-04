use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use proc_macro_error::ResultExt;
use quote::format_ident;
use std::convert::*;

use super::field_type::*;

pub fn as_holder_visitor(input: &syn::Ident) -> syn::Ident {
    format_ident!("{}HolderVisitor", input)
}

pub fn as_holder_ident(input: &syn::Ident) -> syn::Ident {
    format_ident!("{}Holder", input)
}

pub fn as_holder_path(input: &syn::Type) -> syn::Type {
    let ft: FieldType = input
        .clone()
        .try_into()
        .expect_or_abort("as_holder! only accepts espr-generated type");
    ft.into_holder().into()
}

pub fn as_visitor_ident(input: &syn::Ident) -> syn::Ident {
    format_ident!("{}Visitor", input)
}

pub fn serde_crate() -> syn::Path {
    let step_p21 = step_p21_crate();
    syn::parse_quote!( #step_p21::serde )
}

pub fn itertools_crate() -> syn::Path {
    let step_p21 = step_p21_crate();
    syn::parse_quote!( #step_p21::itertools )
}

/// Returns `crate` or `::step_p21` as in step_p21 crate or not
pub fn step_p21_crate() -> syn::Path {
    let path = crate_name("step-p21").unwrap();
    match path {
        FoundCrate::Itself => match std::env::var("CARGO_TARGET_TMPDIR") {
            Ok(_) => {
                // For tests and benches in step_p21 crate
                //
                // https://doc.rust-lang.org/cargo/reference/environment-variables.html
                // > CARGO_TARGET_TMPDIR — Only set when building integration test or benchmark code.
                let mut segments = syn::punctuated::Punctuated::new();
                segments.push(syn::PathSegment {
                    ident: syn::Ident::new("step_p21", Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                syn::Path {
                    leading_colon: Some(syn::token::PathSep::default()),
                    segments,
                }
            }
            Err(_) => {
                let mut segments = syn::punctuated::Punctuated::new();
                segments.push(syn::PathSegment {
                    ident: syn::Ident::new("crate", Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                syn::Path {
                    leading_colon: None,
                    segments,
                }
            }
        },
        FoundCrate::Name(name) => {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push(syn::PathSegment {
                ident: syn::Ident::new(&name, Span::call_site()),
                arguments: syn::PathArguments::None,
            });
            syn::Path {
                leading_colon: Some(syn::token::PathSep::default()),
                segments,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holder_path() {
        let path = syn::parse_str("::some::Struct").unwrap();
        let holder = as_holder_path(&path);
        let ans = syn::parse_str("::some::StructHolder").unwrap();
        assert_eq!(holder, ans);
    }

    #[test]
    fn optional_holder_path() {
        let path = syn::parse_str("Option<::some::Struct>").unwrap();
        let holder = as_holder_path(&path);
        let ans = syn::parse_str("Option<::some::StructHolder>").unwrap();
        assert_eq!(holder, ans);
    }
}
