#![feature(proc_macro_diagnostic)]

#[macro_use]
extern crate lazy_static;
extern crate random_string;

extern crate proc_macro;
extern crate quote;
extern crate syn;

use std::collections::HashMap;
use std::ops::Add;
use std::sync::Mutex;

use proc_macro::TokenStream;
use quote::quote_spanned;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::Ident;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemFn};

struct IndexHolder {
    index: u32,
}

impl IndexHolder {
    fn new() -> Self {
        IndexHolder { index: 0 }
    }

    fn add(&mut self) {
        self.index = self.index.add(1);
    }

    fn get(&self) -> u32 {
        self.index.clone()
    }
}

lazy_static! {
    static ref INDEX: Mutex<IndexHolder> = Mutex::new(IndexHolder::new());
}

fn add_and_return() -> u32 {
    let mut index_lock = INDEX.lock().unwrap();

    index_lock.add();

    index_lock.get()
}

fn just_return() -> u32 {
    let index_lock = INDEX.lock().unwrap();

    index_lock.get()
}

#[proc_macro_derive(ResponseCommon)]
pub fn derive_trait_body_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl samovar::common::ResponseCommon for #name {
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

    let fname = &input.sig.ident;

    let first_arg = &args[0];
    let second_arg = &args[1];

    let first_arg_nv = {
        match first_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let second_arg_nv = {
        match second_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let path_1 = &first_arg_nv.path;
    let path_2 = &second_arg_nv.path;

    let f1_string = quote! {#path_1}.to_string().to_lowercase();
    let f2_string = quote! {#path_2}.to_string().to_lowercase();

    let lit_1 = &first_arg_nv.lit;
    let lit_2 = &second_arg_nv.lit;

    let lits_map = {
        let mut m = HashMap::<String, &syn::Lit>::new();

        m.insert(f1_string, lit_1);
        m.insert(f2_string, lit_2);

        m
    };

    let uri = lits_map.get("path").unwrap();
    let method = lits_map.get("method").unwrap();

    let path_str = quote!(#uri).to_string().to_lowercase();

    let path_str_rep = path_str.replace("\"", "");

    let mut path_str_chars = path_str_rep.chars().collect::<Vec<char>>();

    if path_str_chars.len() > 2 {
        if &path_str_chars[0] == &'/' {
            path_str_chars.remove(0);
        }

        if path_str_chars.last().unwrap() == &'/' {
            path_str_chars.pop();
        }
    }

    let method_str = quote!(#method).to_string().to_lowercase().replace("\"", "");

    let num = format!("{}", add_and_return());

    let endpint_name = format_ident!("endpoint_{}", &num);

    let function_name = format_ident!("getter_{}", &num);

    let expanded = quote! {
        #input

        fn #function_name() -> Box<samovar::endpoint::Endpoint> {
            let #endpint_name = Box::new(samovar::endpoint::Endpoint::new_string_method(#path_str_rep.to_string(),  &#fname, #method_str.to_string()));

            #endpint_name
        }

    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn context(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let input_sig_inputs = input.clone().sig.inputs;

    let ty = match input_sig_inputs.first() {
        Some(i) => match i {
            syn::FnArg::Receiver(_) => todo!(),
            syn::FnArg::Typed(typed) => &typed.ty,
        },
        None => todo!(),
    };

    let first_arg = &args[0];

    let first_arg_nv = {
        match first_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let path_1 = &first_arg_nv.path;

    let f1_string = quote! {#path_1}.to_string().to_lowercase();

    let lit_1 = &first_arg_nv.lit;

    let lits_map = {
        let mut m = HashMap::<String, &syn::Lit>::new();

        m.insert(f1_string, lit_1);

        m
    };

    let ctx_name = lits_map.get("name").unwrap();

    let name_str = quote!(#ctx_name)
        .to_string()
        .to_lowercase()
        .replace("\"", "");

    let ident_add = format_ident!("ctx_{}_insert", name_str);
    let ident_static = format_ident!("CTX_{}", name_str.to_uppercase());

    let init = quote! {
        {
            let mut ctx = std::collections::HashMap::<String, Box<#ty>>::new();

            std::sync::RwLock::new(ctx)
         }
    };

    let init_ptr = quote_spanned! {init.span() =>
        Box::into_raw(Box::new(#init))
    };

    let expanded = quote! {
        pub struct #ident_static;

        impl std::ops::Deref for #ident_static {
            type Target = std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>>;

            fn deref(&self) -> &std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>> {
                static ONCE: std::sync::Once = std::sync::Once::new();
                static mut VALUE: *mut std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>> = 0 as *mut std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>>;

                unsafe {
                    ONCE.call_once(|| VALUE = #init_ptr);
                    &*VALUE
                }
            }
        }

        pub fn #ident_add(key: &str, item: #ty) {
            let mut ctx_locked = #ident_static.write().unwrap();

            let item_mutex = Box::new(item);

            *ctx_locked.insert(key.to_string(), item_mutex);
        }

        #input

    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn memory_session(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let first_arg = &args[0];

    let first_arg_nv = {
        match first_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let lit_1 = &first_arg_nv.lit;

    let name_str = quote!(#lit_1).to_string().to_lowercase().replace("\"", "");

    let ident_add = format_ident!("sess_{}_insert", name_str);
    let ident_static = format_ident!("SESS_{}", name_str.to_uppercase());
    let ident_get = format_ident!("sess_{}_get", name_str);

    let init = quote! {
        {
            let mut sess = samovar::session::MemorySession::new();

            std::sync::RwLock::new(sess)
         }
    };

    let init_ptr = quote_spanned! {init.span() =>
        Box::into_raw(Box::new(#init))
    };

    let expanded = quote! {
        pub struct #ident_static;

        impl std::ops::Deref for #ident_static {
            type Target = std::sync::RwLock<samovar::session::MemorySession>;

            fn deref(&self) -> &std::sync::RwLock<samovar::session::MemorySession> {
                static ONCE: std::sync::Once = std::sync::Once::new();
                static mut VALUE: *mut std::sync::RwLock<samovar::session::MemorySession>  = 0 as *mut std::sync::RwLock<samovar::session::MemorySession>;

                unsafe {
                    ONCE.call_once(|| VALUE = #init_ptr);
                    &*VALUE
                }
            }
        }

        pub fn #ident_add(key: &str, item: &str) {
            let mut sess_locked = #ident_static.write().unwrap();

            sess_locked.insert(key.to_string(), item.to_string());
        }

        pub fn #ident_get(key: &str) -> Option<String> {
            let mut sess_locked = #ident_static.read().unwrap();

            let got = sess_locked.get(&key.to_string());

            got.clone()
        }

        #input
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn physical_session(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let first_arg = &args[0];
    let second_arg = &args[1];

    let first_arg_nv = {
        match first_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let second_arg_nv = {
        match second_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let path_1 = &first_arg_nv.path;
    let path_2 = &second_arg_nv.path;

    let f1_string = quote! {#path_1}.to_string().to_lowercase();
    let f2_string = quote! {#path_2}.to_string().to_lowercase();

    let lit_1 = &first_arg_nv.lit;
    let lit_2 = &second_arg_nv.lit;

    let lits_map = {
        let mut m = HashMap::<String, &syn::Lit>::new();

        m.insert(f1_string, lit_1);
        m.insert(f2_string, lit_2);

        m
    };

    let lit_name = lits_map.get("name").unwrap();
    let lit_file_path = lits_map.get("filepath").unwrap();

    let name_str = quote!(#lit_name)
        .to_string()
        .to_lowercase()
        .replace("\"", "");
    let path_str = quote!(#lit_file_path)
        .to_string()
        .to_lowercase()
        .replace("\"", "");

    let ident_add = format_ident!("sess_{}_insert", name_str);
    let ident_static = format_ident!("SESS_{}", name_str.to_uppercase());
    let ident_get = format_ident!("sess_{}_get", name_str);

    let init = quote! {
        {
            let mut sess = samovar::session::PhysicalSession::new(#path_str.to_string());
            sess.initiate();

            std::sync::RwLock::new(sess)
         }
    };

    let init_ptr = quote_spanned! {init.span()=>
        Box::into_raw(Box::new(#init))
    };

    let expanded = quote! {
        pub struct #ident_static;

        impl std::ops::Deref for #ident_static {
            type Target = std::sync::RwLock<samovar::session::PhysicalSession>;

            fn deref(&self) -> &std::sync::RwLock<samovar::session::PhysicalSession> {
                static ONCE: std::sync::Once = std::sync::Once::new();
                static mut VALUE: *mut std::sync::RwLock<samovar::session::PhysicalSession>  = 0 as *mut std::sync::RwLock<samovar::session::PhysicalSession>;

                unsafe {
                    ONCE.call_once(|| VALUE = #init_ptr);
                    &*VALUE
                }
            }
        }

        pub fn #ident_add(key: &str, item: &str) {
            let mut sess_locked = #ident_static.write().unwrap();

            sess_locked.insert(key.to_string(), item.to_string());
        }

        pub fn #ident_get(key: &str) -> Option<String> {
            let mut sess_locked = #ident_static.read().unwrap();

            let got = sess_locked.get(&key.to_string());

            got.clone()
        }

        #input
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn static_server(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let first_arg = &args[0];
    let second_arg = &args[1];
    let third_arg = &args[2];

    let first_arg_nv = {
        match first_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let second_arg_nv = {
        match second_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let third_arg_nv = {
        match third_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let path_1 = &first_arg_nv.path;
    let path_2 = &second_arg_nv.path;
    let path_3 = &third_arg_nv.path;

    let f1_string = quote! {#path_1}.to_string().to_lowercase();
    let f2_string = quote! {#path_2}.to_string().to_lowercase();
    let f3_string = quote! {#path_3}.to_string().to_lowercase();

    let lit_1 = &first_arg_nv.lit;
    let lit_2 = &second_arg_nv.lit;
    let lit_3 = &third_arg_nv.lit;

    let lits_map = {
        let mut m = HashMap::<String, &syn::Lit>::new();

        m.insert(f1_string, lit_1);
        m.insert(f2_string, lit_2);
        m.insert(f3_string, lit_3);

        m
    };

    let lit_glob = lits_map.get("glob").unwrap();
    let lit_findex = lits_map.get("index_file").unwrap();
    let lit_path = lits_map.get("path").unwrap();

    let glob_str = quote!(#lit_glob)
        .to_string()
        .to_lowercase()
        .replace("\"", "");
    let findex_str = quote!(#lit_findex)
        .to_string()
        .to_lowercase()
        .replace("\"", "");
    let path_str = quote!(#lit_path)
        .to_string()
        .to_lowercase()
        .replace("\"", "");

    let path_desensitized = path_str.replace("/", "_");

    let endpoint_ident = format_ident!("endpoint_{}", &path_desensitized);

    let struct_name = format_ident!("STATIC_SERVER_{}", &path_desensitized);

    let init = quote! {
        {
            let index_file = {
                match #findex_str {
                    "none" => None,
                    _ => Some(#findex_str.to_string()),
                }
            };

            let path_uri = #path_str.to_string();

            let dir_server = samovar::staticserver::DirServer::new(path_uri, #glob_str, index_file);

            std::sync::RwLock::new(dir_server)
         }
    };

    let init_ptr = quote_spanned! {init.span() =>
        Box::into_raw(Box::new(#init))
    };

    let expanded = quote! {
        pub struct #struct_name;

        impl std::ops::Deref for #struct_name {
            type Target = std::sync::RwLock<samovar::staticserver::DirServer>;

            fn deref(&self) -> &std::sync::RwLock<samovar::staticserver::DirServer> {
                static ONCE: std::sync::Once = std::sync::Once::new();
                static mut VALUE: *mut std::sync::RwLock<samovar::staticserver::DirServer> = 0 as *mut std::sync::RwLock<samovar::staticserver::DirServer>;

                unsafe {
                    ONCE.call_once(|| VALUE = #init_ptr);
                    &*VALUE
                }
            }
        }

        #[route(method = "GET", path = #path_str)]
        fn #endpoint_ident(r: &samovar::request::Request) -> samovar::common::ResponseTextWrapper {

            let dir_server = #struct_name.read().unwrap();

            let req_uri = r.get_raw_uri();

            dir_server.compose(req_uri)
        }

        #input
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn samovar(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let first_arg = &args[0];
    let second_arg = &args[1];

    let first_arg_nv = {
        match first_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let second_arg_nv = {
        match second_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => panic!("Must ba named value"),
            },
            _ => panic!("Must be named value"),
        }
    };

    let path_1 = &first_arg_nv.path;
    let path_2 = &second_arg_nv.path;

    let f1_string = quote! {#path_1}.to_string().to_lowercase();
    let f2_string = quote! {#path_2}.to_string().to_lowercase();

    let lit_1 = &first_arg_nv.lit;
    let lit_2 = &second_arg_nv.lit;

    let lits_map = {
        let mut m = HashMap::<String, &syn::Lit>::new();

        m.insert(f1_string, lit_1);
        m.insert(f2_string, lit_2);

        m
    };

    let addr = lits_map.get("addr").unwrap();
    let port = lits_map.get("port").unwrap();

    let addr_str = quote!(#addr).to_string().to_lowercase().replace("\"", "");
    let port_str = quote!(#port).to_string().to_lowercase().replace("\"", "");

    let mut function_names: Vec<Ident> = vec![];

    let max = just_return();

    for i in 1..max {
        let function_name = format_ident!("getter_{}", i);

        function_names.push(function_name);
    }

    let init = quote! {
        {
            let mut sam = samovar::samovar::Samovar::new(#addr_str, #port_str.parse::<u32>().unwrap());

            #(sam.insert_endpoint(#function_names());)*

            std::sync::Mutex::new(sam)
         }
    };

    let init_ptr = quote_spanned! {init.span() =>
        Box::into_raw(Box::new(#init))
    };

    let expanded = quote! {
        pub struct SAMOVAR;

        impl std::ops::Deref for SAMOVAR{
            type Target = std::sync::Mutex<samovar::samovar::Samovar>;

            fn deref(&self) -> &std::sync::Mutex<samovar::samovar::Samovar> {
                static ONCE: std::sync::Once = std::sync::Once::new();
                static mut VALUE: *mut std::sync::Mutex<samovar::samovar::Samovar> = 0 as *mut std::sync::Mutex<samovar::samovar::Samovar>;

                unsafe {
                    ONCE.call_once(|| VALUE = #init_ptr);
                    &*VALUE
                }
            }
        }


        pub fn add_new_endpoint(e: Box<samovar::endpoint::Endpoint>) {
            let mut samovar_lock = SAMOVAR.lock().unwrap();

            samovar_lock.insert_endpoint(e);
        }


        pub fn print_endpoint_uris() {
            let mut samovar_lock = SAMOVAR.lock().unwrap();

            samovar_lock.print_endpoint_uris();

        }

        pub fn serve_forever() {
            let mut samovar_lock = SAMOVAR.lock().unwrap();

            samovar_lock.run()
        }

        #input
    };

    proc_macro::TokenStream::from(expanded)
}

enum Operation {
    CREATE,
    INSERT,
    GET,
    FREE,
}

#[proc_macro]
pub fn context_global(item: TokenStream) -> TokenStream {
    let arg = item.to_string();

    let arg_split = arg.split_whitespace().collect::<Vec<&str>>();

    let operation = {
        let first_word = &arg_split[0].to_lowercase();

        match first_word.as_str() {
            "create" => Operation::CREATE,
            "get" => Operation::GET,
            "insert" => Operation::INSERT,
            "free" => Operation::FREE,
            _ => panic!("Wrong args"),
        }
    };

    let expanded = match operation {
        Operation::CREATE => {
            let name_str = &arg_split[1];
            let type_str = &arg_split[3];

            let ident_static = format_ident!("CTX_{}", name_str.to_uppercase());
            let ty = format_ident!("{}", type_str);

            let init = quote! {
                {
                    let mut ctx = std::collections::HashMap::<String, Box<#ty>>::new();

                    std::sync::RwLock::new(ctx)
                 }
            };

            let init_ptr = quote_spanned! {init.span() =>
                Box::into_raw(Box::new(#init))
            };

            let expanded = quote! {
                pub struct #ident_static;

                impl std::ops::Deref for #ident_static {
                    type Target = std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>>;

                    fn deref(&self) -> &std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>> {
                        static ONCE: std::sync::Once = std::sync::Once::new();
                        static mut VALUE: *mut std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>> = 0 as *mut std::sync::RwLock<std::collections::HashMap<String, Box<#ty>>>;

                        unsafe {
                            ONCE.call_once(|| VALUE = #init_ptr);
                            &*VALUE
                        }
                    }
                }

            };

            expanded
        }
        Operation::INSERT => {
            let value = &arg_split[1];
            let name = &arg_split[3];
            let key = &arg_split[5];

            let value_ident = format_ident!("{}", value);
            let ident_static = format_ident!("CTX_{}", name.to_uppercase());
            let key_str = key.to_string();
            let locked_name = format_ident!("lock_{}", key_str);

            let expanded = quote! {
                let mut #locked_name = #ident_static.write().unwrap();

                #locked_name.insert(#key_str.to_string(), Box::new(#value_ident));

                std::mem::drop(#locked_name);
            };

            expanded
        }
        Operation::GET => {
            let key = &arg_split[1];
            let name = &arg_split[3];

            let unwrap = {
                match arg_split.get(4) {
                    Some(str) => str.to_lowercase() == "unwrap",
                    None => false,
                }
            };

            let ident_static = format_ident!("CTX_{}", name.to_uppercase());
            let key_str = key.to_string();
            let locked_name = format_ident!("lock_{}", key_str);

            let item_got = format_ident!("got_{}", key);

            let expanded_init = quote! {
                let #locked_name = #ident_static.read().unwrap();
            };

            let expanded = {
                match unwrap {
                    true => {
                        quote_spanned! {expanded_init.span()=>
                            #expanded_init

                            let #item_got = #locked_name.get(#key_str).unwrap();
                        }
                    }
                    false => {
                        quote_spanned! {expanded_init.span()=>
                            #expanded_init

                            let #item_got = #locked_name.get(#key_str);
                        }
                    }
                }
            };

            expanded
        }
        Operation::FREE => {
            let key = &arg_split[1];

            let key_str = key.to_string();
            let locked_name = format_ident!("lock_{}", key_str);

            let expanded = quote! {
                std::mem::drop(#locked_name);
            };

            expanded
        }
    };

    proc_macro::TokenStream::from(expanded)
}
