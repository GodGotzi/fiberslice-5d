extern crate proc_macro;

use quote::{__private::Span, quote};
use syn::{parse, DeriveInput, Ident, Type, Variant};

#[proc_macro_derive(TypeEq)]
pub fn type_eq_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(TypeHolder)]
pub fn type_holder_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse(input).unwrap();
    // Get the name of the enum
    let enum_name = &derive_input.ident;

    let variants = match derive_input.data {
        syn::Data::Struct(_) => panic!("Structs are not supported with TypeEq"),
        syn::Data::Enum(data) => data.variants.into_iter().collect::<Vec<Variant>>(),
        syn::Data::Union(_) => panic!("Union are not supported with TypeEq"),
    };

    let types = variants
        .iter()
        .map(|f| {
            let types: Vec<Type> = f.fields.iter().map(|field| field.ty.clone()).collect();

            match types.len() {
                0 => panic!("TypeHolder variants must have at least one field"),
                1 => types[0].clone(),
                _ => panic!("TypeHolder variants must have only one field"),
            }
        })
        .collect::<Vec<Type>>();

    let holder = Ident::new(
        format!("{}Holder", enum_name.to_string()).as_str(),
        Span::call_site(),
    );

    let variables: Vec<Ident> = variants
        .iter()
        .map(|variant| {
            Ident::new(
                format!(
                    "_{}",
                    variant.ident.to_string().as_str().to_lowercase().as_str()
                )
                .as_str(),
                Span::call_site(),
            )
        })
        .collect();

    let variant_idents: Vec<Ident> = variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect();

    // Generate the implementation for `TypeEq` trait
    let stream = quote! {
        pub struct #holder {
            #(#variables: Option<#types>, )*
        }

        impl TryFrom<String> for #enum_name {
            type Error = crate::error::Error;

            fn try_from(s: String) -> Result<Self, Self::Error> {
                let variant = s.split(":").nth(0)?;
                let value = s.split(":").nth(1)?;


                match variant {
                    #(stringify!(#variant_idents) => {
                        let value = value.parse::<#types>()?;

                        Ok(#enum_name::#variant_idents(value))
                    },)*
                }

            }
        }

        impl #holder {

            pub fn empty() -> Self {
                Self {
                    #(#variables: None, )*
                }
            }

            pub fn parse(&mut self, line: &str) {






            }

        }

    };

    // Return the generated implementation as a TokenStream
    proc_macro::TokenStream::from(stream)
}
