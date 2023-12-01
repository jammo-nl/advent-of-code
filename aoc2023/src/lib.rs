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

        use std::time::{Duration, Instant};

        struct MeasureTime {
            start: Instant,
        }
        impl MeasureTime {
            pub fn start() -> Self {
                Self {
                    start: Instant::now(),
                }
            }
            pub fn duration(&self) -> String {
                let duration = self.start.elapsed();
                if duration < Duration::from_millis(1) {
                    format!("{}μs", duration.as_micros())
                } else if duration < Duration::from_secs(1) {
                    format!("{:.3}ms", duration.as_micros() as f64 / 1000.0)
                } else {
                    format!("{:.3}s", duration.as_millis() as f64 / 1000.0)
                }
            }
        }

        #solution
        fn main() {
            let total = MeasureTime::start();
            // get file content and add timing
            let (p1, p2) = solution(INPUT.trim());
            println!("Finished in {}", total.duration());

            println!("Part one: {}", p1);
            println!("Part two: {}", p2);
      }
    };
    TokenStream::from(tokens)
}
