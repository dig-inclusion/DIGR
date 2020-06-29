#[macro_use]
extern crate serde;
extern crate serde_yaml;

use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use smol;
use surf;
use anyhow::{Error};
use scraper::{Html, Selector};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericArgument, PathArguments, Type, TypeNever};


// use std::{thread, time};


#[derive(StructOpt, Debug)]
#[structopt(
    name = "digr",
    about = "an automated accessibility test runner based on rules"
)]
struct Arguments {
    #[structopt(short = "r", long = "rules", help = "Rules folder")]
    rules: PathBuf,

    #[structopt(short = "u", long = "url", help = "Url to test")]
	url: String,

    #[structopt(short = "d", long = "depth", help = " Depth or resources to follow on page", default_value = "0")]	
	depth: u8,
}

struct TestType {
    op: String,
    values: Option<Vec<String>>,
}

#[proc_macro_derive(NotNoneFields)]
pub fn not_none_fields(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let not_none_field_names = input.fields.iter().for_each(|field| {
      if field.ty != syn::TypeNever { field.ident}
    })

    // Build the output
    let output = quote! {
        impl #name {
            pub fn not_none_fields() -> Vec<str> {
                #not_none_field_names
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(output)
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct TestValue {
    name: String,
    r#if: Option<Vec<String>>,
    r#let: Option<String>,
    links: Option<String>,
    ifNotEquals: Option<Vec<String>>,
    ifNotNull: Option<String>,
    assertNotGreaterThan: Option<Vec<String>>,
    assertNotEquals: Option<Vec<String>>,
    assertNotNull: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct ValidationValue {
    name: String,
    case: String,
    assert: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct RuleSpec {
    name: String,
    meta: Option<BTreeMap<String, String>>,
    on: Vec<String>,
    #[serde(default = "default_hidden")]
    includeHidden: bool,
    tests: Vec<TestValue>,
    validations: Vec<ValidationValue>,
}

fn default_hidden() -> bool {
    false
}

fn main() {
    let opts = Arguments::from_args();
    let site_url: &str = &opts.url;

    // println!("{} \n {:?}", site_url, opts);

    let file = File::open(opts.rules).expect("Unable to open file");

    let spec: RuleSpec = serde_yaml::from_reader(file).unwrap();

	// println!("{:?}", spec);
	
	smol::run(async {

		let body = surf::get(site_url)
			.recv_string()
			.await
            .map_err(Error::msg);

		// println!("Site html: {:?}", body);

		let b = match body {
			Ok(html) => html,
			Err(error) => panic!("Problem accessing the url: {:?}", error),
		};

		let page_slice: &str = &b;
        let fragment = Html::parse_fragment(page_slice);
        
        for tag in spec.on.iter() {

            // println!("{:?}", e);

            let selector = Selector::parse(tag).unwrap();

            for element in fragment.select(&selector) {
                println!("{:?}", element);
                
            }

        }
	});

}
