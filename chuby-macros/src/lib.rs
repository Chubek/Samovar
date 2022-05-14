extern crate proc_macro;
extern  crate quote;
extern  crate syn;

use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ResponseCommon)]
pub fn derive_trait_body_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        use chuby_http::common::ResponseCommon;

        impl ResponseCommon for #name {
            fn get_length(&self) -> usize {
                let self_str = serde_json::to_string(&self).unwrap();

                self_str.len()
           }

           fn parse_to_string(&self) -> String {
                let self_str = serde_json::to_string(&self).unwrap();

                self_str
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}



#[proc_macro_derive(ContextCommon)]
pub fn derive_train_contect_common(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let name = input.ident;

    let expanded = quote! {

        impl #impl_generics TrA for #name #ty_generics {
            type Item = #ty_generics;

            fn do_type(&self) -> Item {
                self.mb
            }
        }
    };

    println!("{}", expanded.to_string());

    proc_macro::TokenStream::from(expanded)
}
