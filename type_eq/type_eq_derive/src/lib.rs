extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse, Ident, DeriveInput};

#[proc_macro_derive(TypeEq)]
pub fn type_eq_derive(input: TokenStream) -> TokenStream {

    let derive_input: DeriveInput = parse(input).unwrap();
    // Get the name of the enum
    let enum_name = &derive_input.ident;

    let variants: Vec<Ident> = match derive_input.data {
        syn::Data::Struct(_) => panic!("Structs are not supported with TypeEq"),
        syn::Data::Enum(data) => data.variants.into_iter().map(|f| f.ident).collect(),
        syn::Data::Union(_) => panic!("Union are not supported with TypeEq"),
    };

    // Generate the implementation for `TypeEq` trait
    let expanded = quote! {
        impl TypeEq<#enum_name> for #enum_name {
            fn type_eq(&self, other: #enum_name) -> bool {
                match (self, other) {
                    #((#enum_name::#variants(_), #enum_name::#variants(_)) => true,) *
                    _ => false
                } 
            }
        }
    };

    // Return the generated implementation as a TokenStream
    TokenStream::from(expanded)
}
