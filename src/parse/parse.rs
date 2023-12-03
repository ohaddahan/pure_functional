use proc_macro2::TokenStream as TokenStream2;
use syn::parse::Nothing;
use syn::ItemFn;

pub fn parse(args: TokenStream2, input: TokenStream2) -> syn::Result<ItemFn> {
    let _: Nothing = syn::parse2::<Nothing>(args)?;
    let function: ItemFn = syn::parse2(input)?;
    Ok(function)
}
