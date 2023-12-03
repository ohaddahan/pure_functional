mod analyze;
mod codegen;
mod parse;
extern crate proc_macro;
use crate::analyze::analyze::analyze;
use crate::codegen::codegen::codegen;
use crate::parse::parse::parse;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_attribute]
pub fn pure_functional(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = TokenStream2::from(args);
    let input = TokenStream2::from(input);

    let function = match parse(args, input.clone()) {
        Ok(function) => function,
        Err(parse_error) => {
            let compile_error = parse_error.to_compile_error();
            return quote!(#compile_error #input).into();
        }
    };
    let fn_analysis = match analyze(function) {
        Ok(fn_analysis) => fn_analysis,
        Err(analyze_error) => {
            let compile_error = analyze_error.to_compile_error();
            return quote!(#compile_error #input).into();
        }
    };
    let expanded = codegen(fn_analysis);
    TokenStream::from(expanded)
}
