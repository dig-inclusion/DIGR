use proc_macro::TokenStream;
use quote::quote;
use syn::Path;
use syn::punctuated::Pair;
use syn::token::Colon2;
use syn::{parse_macro_input, DeriveInput, GenericArgument, PathArguments, Type, TypeNever, ItemStruct, PathSegment };

// TODO store (with lazy static) the vec of string
// TODO maybe optimization, reverse the order of segments
// fn extract_option_segment(path: &Path) -> Option<Pair<&PathSegment, &Colon2>> {
//     let idents_of_path = path
//         .segments
//         .iter()
//         .into_iter()
//         .fold(String::new(), |mut acc, v| {
//             acc.push_str(&v.ident.to_string());
//             acc.push('|');
//             acc
//         });
//     vec!["std|option|Option|"]
//         .into_iter()
//         .find(|s| &idents_of_path == *s)
//         .and_then(|_| path.segments.last())
// }

#[proc_macro_derive(NotNoneFields)]
pub fn not_none_fields(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

    // let not_none_field_names = input.fields.iter().for_each(|field| {
    //   if field.ty != TypeNever { field.ident }
    // });

    let not_none_field_names: Vec<String>;
    for field in input.fields.iter() {
        if field.ty != Type::TypeNever { not_none_field_names.push(stringify!(field.ident).to_string()) }
    }

    // Build the output
    let output = quote! {
        impl #name {
            pub fn not_none_fields() -> Vec<String> {
                #not_none_field_names
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
