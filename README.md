# error_spanned

This crate is meant to be used inside a proc macro lib.

This crate provides a trait `ErrorSpanned` and a derive macro `error_spanned_derive::ErrorSpanned`.
The trait enforces that the error type supports conversion to `syn::Error` and `proc_macro2::TokenStream`.

This trait is implemented for a wrapper struct generated by the
`#[derive(ErrorSpanned)]`. The struct generated by it stores line, file and span info.

# Example

For a file named `error_spanned.rs`:

```rust
use std::fmt::Display;
use error_spanned::{ErrorSpanned as _, ErrorSpanned};
use proc_macro2::Span;

// Deriving ErrorSpanned generates a struct named `CustomErrorSpanned`
// and a function-like macro named custom_error!().
#[derive(Debug, ErrorSpanned)]
enum CustomError {
    Error1,
    Error2(String),
    Error3 {
        x: i32,
        y: i32,
    },
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            CustomError::Error1 => write!(f, "Error1"),
            CustomError::Error2(string) => write!(f, "Error2({})", string),
            CustomError::Error3 {x, y} => write!(f, "Error3 {{ x = {}, y = {} }}",x, y),
        }
    }
}
impl std::error::Error for CustomError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Printing the following errors will also print line and file
    println!("{}", custom_error!(Error1, Span::call_site()));
    println!("{}", custom_error!(Error2("Hello world".into()), Span::call_site()));
    println!("{}", custom_error!(Error3{x: 1, y: 2}, Span::call_site()));
    Ok(())
}
```

Output is as follows:
```text
Line: 31, File: /Users/mihir/rust_crates/error_spanned/tests/error_spanned.rs:
Error1
Line: 32, File: /Users/mihir/rust_crates/error_spanned/tests/error_spanned.rs:
Error2(Hello world)
Line: 33, File: /Users/mihir/rust_crates/error_spanned/tests/error_spanned.rs:
Error3 { x = 1, y = 2 }
```
As shown in the above example, errors should be propagated by the generated macro
(like `custom_error!()`). The generated macro is the snake cased version of the error enum.

The generated macro accepts the error variant (not complete path) as the first arg 
and span as the second arg.

The type returned by the macro is `<enum_named>Spanned`. This type can be converted to
`proc_macro2::TokenStream` or `syn::Error` as it implements `ErrorSpanned`.
On conversion to `proc_macro2::TokenStream` or `syn::Error` it preseves span information.
