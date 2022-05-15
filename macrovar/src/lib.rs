extern crate proc_macro;
extern crate quote;
extern crate syn;

use std::iter::FromIterator;

use proc_macro::*;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, AttributeArgs, ItemFn ,ItemMacro};

#[proc_macro_derive(ResponseCommon)]
pub fn derive_trait_body_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl crate::common::ResponseCommon for #name {
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


/*
#[proc_macro_attribute]
pub fn context(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemStruct);

    let 


    let expanded = quote! {
        let method_enum: crate::common::Method = fromat!("{}", method_string).to_lowercase().into();

        let #struct_name = crate::endpoint::Endpoint::new(#uri, &#input, method_enum);

        let mut endpoint_map = crate::common::ENDPOINT_MAP.lock().unwrap();

        endpoint_map.insert(format!("{}", uri_rep), #struct_name);
    };

    proc_macro::TokenStream::from(expanded)

}
*/

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
    
    let struct_name = format_ident!("endpoint_{}_{}", method_string.clone(), uri_rep.clone());


    let expanded = quote! {
        let method_enum: crate::common::Method = fromat!("{}", method_string).to_lowercase().into();

        let #struct_name = crate::endpoint::Endpoint::new(#uri, &#input, method_enum);

        let mut endpoint_map = crate::common::ENDPOINT_MAP.lock().unwrap();

        endpoint_map.insert(format!("{}", uri_rep), #struct_name);
    };

    proc_macro::TokenStream::from(expanded)

}

#[proc_macro]
pub fn run_forever(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMacro);

    let samovar = &input.attrs[0];

    let expanded = quote! {
        use syn::Ident;

        fn run_forever(samovar: &Samovar) {
            for stream in samovar.listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let request = Request::from(&stream);

                        let uri_clone  = request.uri.clone();

                        let uri_replace = uri_clone.replace("/", "_");

                        let endpoint_map = crate::common::ENDPOINT_MAP.lock().unwrap();

                        let endpoint = &endpoint_map[uri_rep];

                        let arc_stream = std::sync::Arc::new(stream);
                        let arc_request = std::sync::Arc::new(request);
                        let arc_context = std::sync::Arc::new(samovar.context);


                        endpoint.serve_response(arc_stream, arc_context, arc_request)


                    },
                    None => println!("No stream")
                }
            }
        }

        run_forever(#samovar);
    };

    proc_macro::TokenStream::from(expanded)

}

