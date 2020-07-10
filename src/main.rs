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
use non_none_fields::*;

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

// struct TestType {
//     op: String,
//     values: Option<Vec<String>>,
// }

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
struct TestValue {
    name: String,
    r#if: Option<Vec<String>>,
    r#let: Option<String>,
    links: Option<String>,
    ifEquals: Option<Vec<String>>,
    ifNotEquals: Option<Vec<String>>,
    ifGreaterThan: Option<Vec<String>>,
    ifLessThan: Option<Vec<String>>,
    ifGreaterThanOrEquals: Option<Vec<String>>,
    ifLessThanOrEquals: Option<Vec<String>>,
    ifNull: Option<String>,
    ifNotNull: Option<String>,
    ifIncludes: Option<String>,

    assert: Option<Vec<String>>,
    assertEquals: Option<Vec<String>>,
    assertGreaterThan: Option<Vec<String>>,
    assertLessThan: Option<Vec<String>>,
    assertGreaterThanOrEquals: Option<Vec<String>>,
    assertLessThanOrEquals: Option<Vec<String>>,
    assertNull: Option<String>,

    assertNotGreaterThan: Option<Vec<String>>,
    assertNotEquals: Option<Vec<String>>,
    assertNotNull: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
struct ValidationValue {
    name: String,
    case: String,
    assert: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
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

// Function for html tags operations
fn tag_ops(fragment: &Html, tag: &std::string::String) {

    let selector = Selector::parse(&tag).expect("Invalid CSS selector.");
    for element in fragment.select(&selector) {

        println!("{:?}", element.value().name);

        
    }

}

//
// Function for attributes operations
fn attr_ops(index: usize, fragment: &Html, tag: &std::string::String) {

    let tg = tag.replace(&['(', '*', ']', '\''][..], "");
    
    let (first, last) = tg.split_at(index);
    
    let mut attrib = first.to_string();
    attrib.push(']');

    let l = &last.replace('=', "");
    let attr_name: &str = &l;

    let selector = Selector::parse(&tg).expect("Invalid CSS selector.");
    for element in fragment.select(&selector) {

        for elem in &element.value().attr(attr_name) {
            println!("{:?}", elem);

        }
        
    }
}

fn main() {
    let opts = Arguments::from_args();
    let site_url: &str = &opts.url;

    // println!("{} \n {:?}", site_url, opts);

    let file = File::open(opts.rules).expect("Unable to open file, please remember file or folder argument with the -f option.");

    let spec: RuleSpec = serde_yaml::from_reader(file).expect("There was an error parsing RuleSpec");

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

            let is_index = tag.find("=");

            match is_index {
                Some(index) => attr_ops(index, &fragment, tag),
                None => tag_ops(&fragment, tag),
            };

        }
	});

}
