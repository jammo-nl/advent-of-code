use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, ItemFn};

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse input
    let attr = parse_macro_input!(attr as Expr);
    let day_attr = attr.to_token_stream().to_string();
    let input_file = format!("../../inputs/day{}.txt", day_attr);

    let solution = parse_macro_input!(item as ItemFn);

    let tokens = quote! {
        const INPUT: &str = include_str!(#input_file);
        #solution
        fn main() {
            // get file content and add timing
            let (p1, p2) = solution(INPUT.trim());
            println!("Part one: {}", p1);
            println!("Part two: {}", p2);
      }
    };
    TokenStream::from(tokens)
}
