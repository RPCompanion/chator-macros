
extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn sha256_to_array(input: TokenStream) -> TokenStream {

    let hex = parse_macro_input!(input as LitStr).value();

    if hex.len() != 64 {
        panic!("Invalid SHA-256 hash length: {}", hex.len());
    }

    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..=i+1], 16).unwrap())
        .collect();

    quote! {
        [ #(# bytes),* ]
    }.into()

}