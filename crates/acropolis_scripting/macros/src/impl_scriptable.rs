use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Lit, LitStr};

fn impl_field_setter(field: &syn::Field) -> TokenStream {
    let name = field.ident.as_ref().unwrap();
    let name_lit = Lit::Str(LitStr::new(&name.to_string(), name.span()));

    quote! {
        #name_lit => println!("Setting field {} to {:?}", #name_lit, value),
    }
}

pub fn impl_scriptable(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let body = match input.data {
        syn::Data::Struct(b) => b,
        _ => panic!("You can only use Scriptable on structs, not enums."),
    };

    let fields = body.fields.iter().map(|f| impl_field_setter(f));

    quote! {
        impl Scriptable for #name {
            fn get_property(&self, property: &str) -> String {
                unimplemented!()
            }

            fn set_property(&mut self, property: &str, value: String) {
                match property {
                    #(#fields)*
                    _ => (),
                }
            }
        }
    }
}
