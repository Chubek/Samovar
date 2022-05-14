#[macro_use]
extern crate chuby_macros;

pub trait TrA {
    type Item;

    fn do_type(&self) -> Self::Item;
}


#[derive(ContextCommon)]
pub struct MB<T> {
    mb: T,
}



fn main() {
    let mb = MB { mb: "sfsf".to_string() };

    println!("{:?}", mb);
}
