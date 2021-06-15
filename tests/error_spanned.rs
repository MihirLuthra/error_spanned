use std::fmt::Display;

use error_spanned_derive::ErrorSpanned;
use proc_macro2::Span;

// Deriving ErrorSpanned generates a struct named `CustomErrorSpanned`
// and a function-like macro named custom_error!().
#[derive(Debug, ErrorSpanned)]
enum CustomError {
    Error1,
    Error2(String),
    Error3 { x: i32, y: i32 },
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            CustomError::Error1 => write!(f, "Error1"),
            CustomError::Error2(string) => write!(f, "Error2({})", string),
            CustomError::Error3 { x, y } => write!(f, "Error3 {{ x = {}, y = {} }}", x, y),
        }
    }
}
impl std::error::Error for CustomError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Printing the following errors will also print line and file
    println!("{}", custom_error!(Error1, Span::call_site()));
    println!(
        "{}",
        custom_error!(Error2("Hello world".into()), Span::call_site())
    );
    println!(
        "{}",
        custom_error!(Error3 { x: 1, y: 2 }, Span::call_site())
    );
    Ok(())
}
