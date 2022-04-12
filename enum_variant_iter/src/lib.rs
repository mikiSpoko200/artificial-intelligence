use std::path::Iter;

use proc_macro::{TokenStream};
use syn::{Data, DeriveInput, parse_macro_input};
use quote::{quote, format_ident};


#[proc_macro_derive(EnumVariantIter)]
pub fn enum_iter(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let enum_name = input_ast.ident;

    if let syn::Data::Enum(enum_tokens) = input_ast.data {
        let _variants = enum_tokens.variants.iter();
        quote!(
            impl EnumVariantIter<T> for #enum_name
            where T: Iterator<Item=Self>
            {
                fn variants() -> T {
                    _variants
                }
            }
        ).into()
    } else {
        panic!("EnumVariantIter can only be implemented on Enums.")
    }
}