use crate::analyze::analyze::FnAnalysis;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::fs;
use std::fs::File;
use std::io::Write;
use syn::parse_quote;

pub fn codegen(fn_analysis: FnAnalysis) -> TokenStream2 {
    let mut inner_func = fn_analysis.function.clone();
    inner_func.vis = parse_quote!(pub);
    let visibility = fn_analysis.function.vis.clone().to_token_stream();
    let signature = fn_analysis.function.sig.clone().to_token_stream();
    let arg_names = fn_analysis.arg_names;
    let args: proc_macro2::TokenStream = arg_names.join(", ").parse().unwrap();
    let sig = fn_analysis.function.sig.ident.clone().to_token_stream();

    let output = quote!(
        #visibility #signature {
            mod inner_mod {
                #inner_func
            }
            inner_mod::#sig(#args)
        }
    );
    // debug(format!("{}_x.txt", sig.to_string()).as_str(), &output);
    output
}

fn debug(name: &str, stream: &proc_macro2::TokenStream) {
    println!("here 1");
    let mut file = File::create(name).unwrap();
    println!("here 2");
    write!(file, "{stream}").unwrap();
    println!("here 3");
    drop(file);

    let input = fs::read_to_string(name).unwrap();
    let syntax_tree = syn::parse_file(&input).unwrap();
    println!("here 4");
    let formatted = prettyplease::unparse(&syntax_tree);
    println!("here 5");
    print!("{}", formatted);
    println!("here 6");
}
