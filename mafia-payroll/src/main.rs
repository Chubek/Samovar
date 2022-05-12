#[macro_use]
extern crate route;


#[route(hell1o, a, "wo1rld", "sss")]
fn aa() -> u8 {
    11 > 2
}

fn main() {
    dummy()
}
