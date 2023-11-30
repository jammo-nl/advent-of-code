use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse input

    let mut solution = parse_macro_input!(input as ItemFn);

    let tokens = quote! {
        #solution
        fn main() {
            // get file content and add timing
            let (p1, p2) = solution("");
            println!("Part one: {}", p1);
            println!("Part two: {}", p2);
      }
    };
    TokenStream::from(tokens)
}
