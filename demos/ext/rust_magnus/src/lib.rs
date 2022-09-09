use magnus::{define_global_function, function, Error};

pub fn reverse(input: String) -> String {
    input.chars().rev().collect()
}

#[magnus::init]
fn init() -> Result<(), Error> {
    define_global_function("reverse_it_rust_magnus", function!(reverse, 1));
    Ok(())
}
