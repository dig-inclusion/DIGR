extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Path;
use syn;
use syn::{parse_macro_input, Type};

#[proc_macro_derive(NonNoneFields)]
pub fn derive_non_none_fields(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as syn::ItemStruct);

    let name = &input.ident;

    let mut field_names = Vec::new();
    for field in input.fields.iter() {

        match &field.ty {
            Type::Path(typepath) if typepath.qself.is_none() => {
                let fname = field.ident.as_ref().expect("tuple structs are not supported").to_string();

                if path_is_option(&typepath.path) {
                    let ident = &field.ident;
                    field_names.push(quote! {
                        if self.#ident.is_some() {
                            v.push(#fname);
                        }
                    });
                } else {
                    field_names.push(quote! {
                        v.push(#fname);
                    });
                }
            },
            _ => (),
        }
    }

    let output = quote! {
        impl #name {
            pub fn non_none_fields(&self) -> Vec<&'static str>{
                let mut v = vec![];
                #(#field_names)*
                v
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn path_is_option(path: &Path) -> bool {
    path.leading_colon.is_none()
    && path.segments.len() == 1
    && path.segments.iter().next().unwrap().ident == "Option"
}