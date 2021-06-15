use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Ident};

#[proc_macro_derive(ErrorSpanned)]
pub fn error_spanned_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    match derive_input.data {
        Data::Enum(..) => {}
        _ => {
            return Error::new(Span::call_site(), "Expected enum")
                .to_compile_error()
                .into()
        }
    };

    generate_spanned_wrapper(&derive_input).into()
}

fn generate_spanned_wrapper(derive_input: &DeriveInput) -> TokenStream2 {
    let vis = &derive_input.vis;
    let name = &derive_input.ident;
    let name_snake_case = name.to_string().to_case(Case::Snake);
    let name_snake_case_ident = Ident::new(&name_snake_case, name.span());
    let wrapper_name = format!("{}Spanned", name);
    let wrapper_name_ident = Ident::new(&wrapper_name, name.span());

    let wrapper_doc_comments = format!(
        "This is a wrapper around [`{}`] adding, line, file and span info",
        name
    );

    quote! {
        #[doc = #wrapper_doc_comments]
        #[derive(Debug)]
        #vis struct #wrapper_name_ident {
            error: #name,
            line: u32,
            file: &'static str,
            span: proc_macro2::Span,
        }

        impl std::ops::Deref for #wrapper_name_ident {
            type Target = #name;

            fn deref(&self) -> &Self::Target {
                &self.error
            }
        }

        impl std::ops::DerefMut for #wrapper_name_ident {
            fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
                &mut self.error
            }
        }

        impl std::fmt::Display for #wrapper_name_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
                write!(
                    f,
                    "Line: {}, File: {}:\n{}",
                    self.line,
                    self.file,
                    self.error.to_string(),
                )
            }
        }

        impl std::convert::Into<syn::Error> for #wrapper_name_ident {
            fn into(self) -> syn::Error {
                syn::Error::new(self.span, self.to_string())
            }
        }
        impl std::convert::Into<proc_macro2::TokenStream> for #wrapper_name_ident {
            fn into(self) -> proc_macro2::TokenStream {
                std::convert::Into::<syn::Error>::into(self).to_compile_error().into()
            }
        }

        impl #wrapper_name_ident {
            #vis fn new(error: #name, line: u32, file: &'static str, span: proc_macro2::Span) -> Self {

                fn assert_to_string_impl<T: std::string::ToString>() {}
                assert_to_string_impl::<#name>();

                Self {
                    error,
                    line,
                    file,
                    span,
                }
            }
        }

        impl error_spanned::ErrorSpanned for #wrapper_name_ident {}

        impl std::error::Error for #wrapper_name_ident {}

        /// A macro that adds [`std::line`], [`std::file`]
        /// and [`proc_macro2::Span`] information to the error
        macro_rules! #name_snake_case_ident {
            ($err_type:tt($($args:tt)+), $span:expr) => {
                #wrapper_name_ident::new(
                    #name::$err_type($($args)+),
                    line!(),
                    file!(),
                    $span,
                )
            };
            ($err_type:tt{ $($args:tt)+ }, $span:expr) => {
                #wrapper_name_ident::new(
                    #name::$err_type { $($args)+ },
                    line!(),
                    file!(),
                    $span,
                )
            };
            ($err_type:tt, $span:expr) => {
                #wrapper_name_ident::new(
                    #name::$err_type,
                    line!(),
                    file!(),
                    $span,
                )
            };
        }

    }
}
