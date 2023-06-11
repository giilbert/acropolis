mod impl_scriptable;

use impl_scriptable::impl_scriptable;
use proc_macro2::{Ident, Span};
use quote::quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;

#[proc_macro_derive(Scriptable)]
pub fn scriptable(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_scriptable(ast).into()
}

#[proc_macro_attribute]
pub fn glued_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: TokenStream2 = item.into();
    let attr: TokenStream2 = attr.into();
    let is_fast = attr.to_string().contains("fast");

    let tokens = if is_fast {
        quote! {
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
            #[cfg_attr(not(target_arch = "wasm32"), deno_core::op(fast))]
            #item
        }
    } else {
        quote! {
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
            #[cfg_attr(not(target_arch = "wasm32"), deno_core::op)]
            #item
        }
    };

    return tokens.into();
}
