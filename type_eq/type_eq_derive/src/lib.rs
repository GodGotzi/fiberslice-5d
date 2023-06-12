extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse, Ident, DeriveInput};

#[proc_macro_derive(TypeEq)]
pub fn type_eq_derive(input: TokenStream) -> TokenStream {


    let ast = parse(input).unwrap();

    impl_type_eq(ast)

}

fn impl_type_eq(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    
    let variants_information: Vec<(usize, Ident)> = match ast.data {
        syn::Data::Struct(_) => panic!("Structs are not supported with TypeEq"),
        syn::Data::Enum(data) => data.variants.into_iter().map(|f| (f.fields.len(), f.ident)).collect(),
        syn::Data::Union(_) => panic!("Union are not supported with TypeEq"),
    };

    let variants_field_len: Vec<usize> = variants_information.iter().map(|f| f.0).collect();
    let variants_ident: Vec<Ident> = variants_information.iter().map(|f| f.1.clone()).collect();

    let gen = quote! {
        impl TypeEq for #name {
            fn type_eq(&self, other: dyn TypeEq) -> bool {
                matches!(
                    (self, other),
                    #((#name::#variants_ident(_), #name::#variants_ident(_)), )*
                )
            }
        }
    };

    gen.into()
}

/*
        matches!(
            (self, item),

            (Item::ToolbarWidth(_), Item::ToolbarWidth(_)) | 
            (Item::SettingsWidth(_), Item::SettingsWidth(_)) | 
            (Item::LayerValue(_), Item::LayerValue(_)) | 
            (Item::TimeValue(_), Item::TimeValue(_)) | 
            (Item::Mode(_), Item::Mode(_))
        )
        
         */