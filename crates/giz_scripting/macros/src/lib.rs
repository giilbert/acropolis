use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Scriptable)]
pub fn scriptable(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    todo!();
}
