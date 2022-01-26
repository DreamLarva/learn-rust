use attribute_macro::route;

#[route(GET, "/")]
pub fn macro_attribute() {
    println!("fn macro_attribute")
}

fn main() {
    macro_attribute()
}
