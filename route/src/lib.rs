extern crate proc_macro;
use proc_macro::*;

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



