use proc_macro::TokenStream;
use quote::quote;
use syn::Path;
use syn::{parse_macro_input, Type, ItemStruct };

fn path_is_not_none(path: &Path) -> bool {
        path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident != "None"
}

#[proc_macro_derive(NotNoneFields)]
pub fn non_none_fields(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

    // let not_none_field_names = input.fields.iter().for_each(|field| {
    //   if field.ty != TypeNever { field.ident }
    // });

    let not_none_field_names: Vec<String>;
    let mut array: [String; 10];
    for field in input.fields.iter() {
        // if field.ty != None { not_none_field_names.push(stringify!(field.ident).to_string()) }

        match field.ty {
            Type::Path(typepath) if typepath.qself.is_none() && path_is_not_none(&typepath.path) => {
                not_none_field_names.push(stringify!(field.ident).to_string())
            }
        }
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

    #[derive(NotNoneFields)]
    struct SampleTest {
        first: str,
        sec: Option<u8>,
        thr: Option<u8>
    }

    #[test]
    fn it_works() {
        let samp = SampleTest("hellp", Some(1), None);
        let nn_flds = samp.non_none_fields();
        assert_eq!(2 + 2, 4);
    }
}
