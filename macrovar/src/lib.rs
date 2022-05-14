extern crate proc_macro;
extern crate quote;
extern crate syn;

use std::iter::FromIterator;

use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, AttributeArgs, ItemFn};

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



#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let method = &args[0];
    let uri = &args[1];

    let method_string = quote!{#method}.to_string();
    let uri_to_string  = quote!{#uri}.to_string();

    let mut uri_chars: Vec<char> = uri_to_string.chars().collect();

    if &uri_chars[0] == &'/' {
        uri_chars.remove(0);
    }

    if &uri_chars.last().unwrap() == &&'/' {
        uri_chars.pop();
    }

    let uri_fin = String::from_iter(uri_chars);

    let uri_rep = uri_fin.replace("/", "_");
    
    let struct_name = format!("endpoint_{}_{}", method_string.clone(), uri_rep.clone());


    let expanded = quote! {
        let method_enum: Method = String::from(#method_string).to_lowercase().into();

        let #struct_name = Endpoint::new(#uri, #input, method_enum);
    };

    proc_macro::TokenStream::from(expanded)

}