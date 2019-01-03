extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let bit = format!("let {}_with_suffix = \"{}\";",
                      input.to_string(), "Hello, World");
    bit.parse().unwrap()
}
