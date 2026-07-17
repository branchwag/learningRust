use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate. parse_macro_input! turns a parse
    // failure into a compile_error! instead of panicking.
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
