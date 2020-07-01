use proc_macro::{TokenStream};
use quote::{quote};
use syn::Path;
use syn;
use syn::{parse_macro_input, Type};

fn path_is_not_none(path: &Path) -> bool {
        path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident != "None"
}

#[proc_macro_derive(NotNoneFields)]
pub fn non_none_fields(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as &syn::ItemStruct);
    // let item: syn::Item = syn::parse(input).expect("failed to parse input");


    let name = &input.ident;

    // let not_none_field_names = input.fields.iter().for_each(|field| {
    //   if field.ty != TypeNever { field.ident }
    // });

    // impl ToTokens for Vec {
    //     fn to_tokens(&self, tokens: &mut TokenStream) {
    //         let d = self.0.iter().map(|datum| datum.0);
    //         tokens.extend(quote! {
    //             Data(<[Datum]>::into_vec(Box::new([ #( Datum(#d) ),* ])))
    //         })
    //     }
    // }
    // let not_none_field_names: Vec<String>;
    // TokenStream::from_iter(vec![
    //     TokenTree::from(Ident::new("let", span)),
    //     TokenTree::from(Ident::new("field_names", span)),
    // ]);
    let mut field_names: Vec<String> = Vec::new();
    for field in input.fields.iter() {
        // if field.ty != None { not_none_field_names.push(stringify!(field.ident).to_string()) }

        match &field.ty {
            Type::Path(typepath) if typepath.qself.is_none() && path_is_not_none(&typepath.path) => {
                field_names.push(stringify!(field.ident).to_string());
            },
            _ => (),
        }
    }

    // Build the output
    let output = quote! {
        impl #name {
            pub fn not_none_fields() -> [String]{
                #(field_names)
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(output)
}
