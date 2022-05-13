extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("{:#?} {:#?}", args, input);

    let b = r#"
        fn dummy() {
            println!("Hello world!");
        }
    "#;

    b.parse().expect("Ff")

}   

#[proc_macro_derive(ResponseBodyType)]
pub fn derive_trait_body_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl ResponseBodyType for #name {
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